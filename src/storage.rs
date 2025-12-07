use character::Character;
use rusqlite::{Connection, params};

#[derive(Debug, Clone)]
pub struct CharacterSummary {
    pub id: i64,
    pub name: String,
}

pub struct CharacterStore {
    conn: Connection,
}

impl CharacterStore {
    pub fn open(path: &str) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        let store = Self { conn };
        store.init_schema()?;
        Ok(store)
    }

    pub fn list_characters(&self) -> rusqlite::Result<Vec<CharacterSummary>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name FROM characters ORDER BY id DESC")?;
        let rows = stmt
            .query_map([], |row| {
                Ok(CharacterSummary {
                    id: row.get(0)?,
                    name: row.get(1)?,
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
        Ok(())
    }
}
