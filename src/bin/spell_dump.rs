use vitality::spell::all_spell_records;

fn main() {
    let mut spells = all_spell_records();
    println!(
        "{:<20} {:<7} {:<10} {:<10} {:>5} {:>5}  {:<10} {:<30} {:<20} {:<20}",
        "slug", "element", "rank", "role", "mana", "cd", "duration", "behaviors", "strong_vs", "weak_vs"
    );
    println!("{:-<180}", "");
    for record in spells.drain(..) {
        println!(
            "{:<20} {:<7} {:<10} {:<10} {:>5} {:>5}  {:<10} {:<30} {:<20} {:<20}",
            record.slug,
            record.element.code(),
            record.minimum_rank.code(),
            record.profile.role,
            record.mana_cost,
            record.base_cooldown,
            record.duration.label(),
            format_behaviors(&record.behaviors),
            format_elements(&record.strong_against),
            format_elements(&record.weak_against)
        );
    }
}

fn format_elements(elements: &[vitality::core::VitalityElement]) -> String {
    if elements.is_empty() {
        return "-".into();
    }
    elements
        .iter()
        .map(|el| el.code())
        .collect::<Vec<_>>()
        .join(",")
}

fn format_behaviors(behaviors: &[vitality::spell::SpellBehavior]) -> String {
    if behaviors.is_empty() {
        return "-".into();
    }
    behaviors
        .iter()
        .map(|behavior| match behavior {
            vitality::spell::SpellBehavior::Attack(kind) => format!("Atk::{:?}", kind),
            vitality::spell::SpellBehavior::Defense(kind) => format!("Def::{:?}", kind),
            vitality::spell::SpellBehavior::Utility(kind) => format!("Util::{:?}", kind),
            vitality::spell::SpellBehavior::Support(kind) => format!("Sup::{:?}", kind),
        })
        .collect::<Vec<_>>()
        .join("|")
}
