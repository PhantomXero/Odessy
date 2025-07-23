use character::Character;
fn main() {
    let mut new_character = Character::new();
    Character::ShowCharacterProfile(&mut new_character);
    let character = Character::level_up(&mut new_character);
    Character::ShowCharacterProfile(&mut new_character);
}
