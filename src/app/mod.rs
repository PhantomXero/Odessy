use character::Character;

use crate::persistence::CharacterStore;
use crate::ui::{character_selector, profile_screen, MenuSelection, ProfileAction};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let store = CharacterStore::open("odessy_characters.db")?;
    let mut rows = store.list_characters()?;
    let selection = loop {
        let selection = character_selector(&rows)?;
        match selection {
            MenuSelection::DeleteProfile(id) => {
                store.delete_character(id)?;
                println!("Deleted profile {id}.");
                rows = store.list_characters()?;
            }
            other => break other,
        }
    };

    let (mut character, mut active_id) = match selection {
        MenuSelection::CreateNew => {
            let character = Character::from_prompt();
            let id = store.save_character(&character)?;
            (character, Some(id))
        }
        MenuSelection::LoadExisting(id) => {
            if let Some(character) = store.load_character(id)? {
                (character, Some(id))
            } else {
                println!("Save not found. Creating a new character.");
                let character = Character::from_prompt();
                let id = store.save_character(&character)?;
                (character, Some(id))
            }
        }
        MenuSelection::DeleteProfile(id) => {
            unreachable!("Delete option ({id}) should be handled before entering the main flow")
        }
    };

    loop {
        match profile_screen(&character)? {
            ProfileAction::Quit => break,
            ProfileAction::LevelUp => character.level_up(),
            ProfileAction::EditIdentity => character.edit_identity(),
            ProfileAction::EditHistory => character.edit_history(),
            ProfileAction::EditPhysical => character.edit_physical(),
            ProfileAction::EditVitality => character.edit_vitality(),
            ProfileAction::EditWarrior => character.edit_warrior(),
            ProfileAction::Save => {
                if let Some(id) = active_id {
                    store.update_character(id, &character)?;
                } else {
                    let id = store.save_character(&character)?;
                    active_id = Some(id);
                }
                println!("Character saved.");
            }
        }
    }

    Ok(())
}
