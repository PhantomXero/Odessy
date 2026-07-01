use character::Character;
use rusqlite::{params, Connection};
use serde_json;
use std::convert::TryFrom;
use vitality::spell::{all_spell_records, SpellRecord};

#[derive(Debug, Clone)]
pub struct CharacterSummary {
    pub id: i64,
    pub name: String,
    pub badges: Vec<String>,
}

impl CharacterSummary {
    pub fn menu_label(&self) -> String {
        if self.badges.is_empty() {
            format!("{} (ID {})", self.name, self.id)
        } else {
            format!(
                "{} (ID {}) – {}",
                self.name,
                self.id,
                self.badges.join(" | ")
            )
        }
    }
}

pub struct CharacterStore {
    conn: Connection,
}

impl CharacterStore {
    pub fn open(path: &str) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        let mut store = Self { conn };
        store.init_schema()?;
        let _ = store.sync_spell_catalog()?;
        Ok(store)
    }

    pub fn list_characters(&self) -> rusqlite::Result<Vec<CharacterSummary>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, payload FROM characters ORDER BY id DESC")?;
        let rows = stmt
            .query_map([], |row| {
                let payload: String = row.get(2)?;
                Ok(CharacterSummary {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    badges: character_badges(&payload),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn save_character(&self, character: &Character) -> rusqlite::Result<i64> {
        let payload = serde_json::to_string(character).expect("character serialization failed");
        let name = character.personal().full_name();
        self.conn.execute(
            "INSERT INTO characters (name, payload) VALUES (?1, ?2)",
            params![name, payload],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn update_character(&self, id: i64, character: &Character) -> rusqlite::Result<()> {
        let payload = serde_json::to_string(character).expect("character serialization failed");
        let name = character.personal().full_name();
        self.conn.execute(
            "UPDATE characters SET name = ?1, payload = ?2 WHERE id = ?3",
            params![name, payload, id],
        )?;
        Ok(())
    }

    pub fn load_character(&self, id: i64) -> rusqlite::Result<Option<Character>> {
        let mut stmt = self
            .conn
            .prepare("SELECT payload FROM characters WHERE id = ?1")?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            let payload: String = row.get(0)?;
            let character: Character = serde_json::from_str(&payload).map_err(|err| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(err),
                )
            })?;
            Ok(Some(character))
        } else {
            Ok(None)
        }
    }

    pub fn delete_character(&self, id: i64) -> rusqlite::Result<()> {
        self.conn
            .execute("DELETE FROM characters WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn list_spells(&self) -> rusqlite::Result<Vec<SpellRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, slug, name, element, minimum_rank, mana_cost, base_cooldown, profile_json, description, strong_against, weak_against
                     , duration, behaviors_json
             FROM spells
             ORDER BY element, minimum_rank, name",
        )?;

        let rows = stmt
            .query_map([], |row| SpellRecord::try_from(row))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn sync_spell_catalog(&mut self) -> rusqlite::Result<usize> {
        let spells = all_spell_records();
        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO spells (slug, name, element, minimum_rank, mana_cost, base_cooldown, profile_json, description, strong_against, weak_against, duration, behaviors_json)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
                 ON CONFLICT(slug) DO UPDATE SET
                    name = excluded.name,
                    element = excluded.element,
                    minimum_rank = excluded.minimum_rank,
                    mana_cost = excluded.mana_cost,
                    base_cooldown = excluded.base_cooldown,
                    profile_json = excluded.profile_json,
                    description = excluded.description,
                    strong_against = excluded.strong_against,
                    weak_against = excluded.weak_against,
                    duration = excluded.duration,
                    behaviors_json = excluded.behaviors_json",
            )?;

            for spell in &spells {
                let profile_json = serde_json::to_string(&spell.profile)
                    .expect("failed to serialize spell profile");
                let strong_json = serde_json::to_string(&spell.strong_against)
                    .expect("failed to serialize spell strengths");
                let weak_json = serde_json::to_string(&spell.weak_against)
                    .expect("failed to serialize spell weaknesses");
                let duration_json = serde_json::to_string(&spell.duration)
                    .expect("failed to serialize spell duration");
                let behaviors_json = serde_json::to_string(&spell.behaviors)
                    .expect("failed to serialize spell behaviors");

                stmt.execute(params![
                    spell.slug,
                    spell.name,
                    spell.element.code(),
                    spell.minimum_rank.code(),
                    spell.mana_cost,
                    spell.base_cooldown,
                    profile_json,
                    spell.description,
                    strong_json,
                    weak_json,
                    duration_json,
                    behaviors_json,
                ])?;
            }
        }
        tx.commit()?;
        Ok(spells.len())
    }

    fn init_schema(&self) -> rusqlite::Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS characters (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                payload TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS spells (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                slug TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                element TEXT NOT NULL,
                minimum_rank TEXT NOT NULL,
                mana_cost INTEGER NOT NULL,
                base_cooldown INTEGER NOT NULL,
                profile_json TEXT NOT NULL,
                description TEXT NOT NULL,
                strong_against TEXT NOT NULL,
                weak_against TEXT NOT NULL,
                duration TEXT NOT NULL,
                behaviors_json TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        self.ensure_spell_column("duration", "TEXT NOT NULL DEFAULT '\"Instant\"'")?;
        self.ensure_spell_column("behaviors_json", "TEXT NOT NULL DEFAULT '[]'")?;
        Ok(())
    }

    fn ensure_spell_column(&self, column: &str, definition: &str) -> rusqlite::Result<()> {
        let sql = format!("ALTER TABLE spells ADD COLUMN {column} {definition}");
        match self.conn.execute(&sql, []) {
            Ok(_) => Ok(()),
            Err(err) => {
                if let rusqlite::Error::SqliteFailure(_, Some(message)) = &err {
                    if message.contains("duplicate column name") {
                        return Ok(());
                    }
                }
                Err(err)
            }
        }
    }
}

fn character_badges(payload: &str) -> Vec<String> {
    let mut badges = Vec::new();
    let Ok(character) = serde_json::from_str::<Character>(payload) else {
        return badges;
    };

    let element = character.vitality().profile().element().to_string();
    badges.push(element);

    if let Some(warrior) = character.warrior() {
        badges.push(warrior.weapon_badge());
    }

    badges.push(character.civic().social_class().to_string());
    badges.push(character.physical().skin_colour().to_string());
    badges.push(character.physical().dominant_hand().to_string());

    badges
}
