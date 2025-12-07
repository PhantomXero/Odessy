mod storage;
mod ui;

use character::Character;
use storage::CharacterStore;
use ui::{MenuSelection, ProfileAction};

fn main() {
    if let Err(err) = run_app() {
        eprintln!("Application error: {err}");
    }
}

fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let store = CharacterStore::open("odessy_characters.db")?;
    let rows = store.list_characters()?;
    let selection = ui::character_selector(&rows)?;

    let mut character = match selection {
        MenuSelection::CreateNew => {
            let character = Character::from_prompt();
            store.save_character(&character)?;
            character
        }
        MenuSelection::LoadExisting(id) => {
            if let Some(character) = store.load_character(id)? {
                character
            } else {
                println!("Save not found. Creating a new character.");
                let character = Character::from_prompt();
                store.save_character(&character)?;
                character
            }
        }
    };

    loop {
        match ui::profile_screen(&character)? {
            ProfileAction::Quit => break,
            ProfileAction::LevelUp => character.level_up(),
            ProfileAction::EditIdentity => character.edit_identity(),
            ProfileAction::EditHistory => character.edit_history(),
            ProfileAction::EditPhysical => character.edit_physical(),
            ProfileAction::EditVitality => character.edit_vitality(),
            ProfileAction::EditWarrior => character.edit_warrior(),
            ProfileAction::Save => {
                store.save_character(&character)?;
                println!("Character saved.");
            }
        }
    }

    Ok(())
}
