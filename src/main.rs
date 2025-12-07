use character::Character;

fn main() {
    let mut new_character = Character::from_prompt();
    new_character.ShowCharacterProfile();
    println!("\n\n");
    new_character.level_up();
    new_character.ShowCharacterProfile();
    println!("\n\n");
    new_character.level_up();
    new_character.ShowCharacterProfile();
    println!("\n\n");
    new_character.level_up();
    new_character.ShowCharacterProfile();
    println!("\n\n");
    new_character.level_up();
    new_character.ShowCharacterProfile();
    println!("\n\n");
    new_character.level_up();
    new_character.ShowCharacterProfile();
    println!("\n\n");
    new_character.level_up();
    new_character.ShowCharacterProfile();
    println!("\n\n");
    new_character.level_up();
    new_character.ShowCharacterProfile();
}
