use crate::prompt::{select_from_menu, MenuItem};
use crate::vitality::{SpellCombatPreference, VitalityAttributes, VitalityElement, VitalityInfo};
use el_roi::read_int;

const VITALITY_GUIDE: &str = "Vitality tracks supernatural growth. Pick an element now so future spell trees know where to plug in, then grow control/power alongside your physical stats.";

pub fn prompt_vitality_info() -> VitalityInfo {
    println!("--- Vitality ---");
    println!("{VITALITY_GUIDE}");
    let element = prompt_element_choice();
    let attributes = allocate_vitality_points();
    VitalityInfo::starter(element, attributes, SpellCombatPreference::Offensive)
}

fn prompt_element_choice() -> VitalityElement {
    let element_entries: Vec<(VitalityElement, MenuItem)> = VitalityElement::all()
        .iter()
        .map(|element| {
            (
                *element,
                MenuItem::with_info(element.label(), element_info(*element)),
            )
        })
        .collect();
    let menu_items: Vec<MenuItem> = element_entries
        .iter()
        .map(|(_, item)| item.clone())
        .collect();
    let selection = select_from_menu("Vitality Elements", Some(VITALITY_GUIDE), &menu_items);
    let element = element_entries[selection.index].0;
    println!("Element: {}", element.label());
    element
}

fn allocate_vitality_points() -> VitalityAttributes {
    const TOTAL: u8 = VitalityAttributes::STARTER_ALLOCATION;
    println!(
        "Allocate {} starter points across Control, Power, and Range for your spell focus.",
        TOTAL
    );
    loop {
        let mut remaining = TOTAL;
        let control = prompt_point_allocation("Control", remaining);
        remaining = remaining.saturating_sub(control);
        let power = prompt_point_allocation("Power", remaining);
        remaining = remaining.saturating_sub(power);
        let range = prompt_point_allocation("Range", remaining);
        let spent = control + power + range;
        if spent != TOTAL {
            println!(
                "You assigned {} points but must spend exactly {}. Let's try again.",
                spent, TOTAL
            );
            continue;
        }
        println!(
            "Control {} | Power {} | Range {} (Total {})",
            control, power, range, TOTAL
        );
        if confirm_allocation(control, power, range) {
            return VitalityAttributes::from_points(control, power, range);
        }
    }
}

fn prompt_point_allocation(label: &str, max: u8) -> u8 {
    loop {
        println!("Assign points to {label} (0-{max}): ");
        let value = read_int(&format!("{label}: "));
        if value < 0 {
            println!("Points cannot be negative.");
            continue;
        }
        let value = value as u16;
        if value > u16::from(max) {
            println!("Maximum available for {label} is {max}. Try again.");
            continue;
        }
        return value as u8;
    }
}

fn confirm_allocation(control: u8, power: u8, range: u8) -> bool {
    let summary = format!("Confirm allocation -> Control {} | Power {} | Range {}", control, power, range);
    let options = vec![
        MenuItem::with_info("Confirm", "Lock these stats in."),
        MenuItem::with_info("Reassign", "Return to allocation and try another spread."),
    ];
    let selection = select_from_menu(&summary, None, &options);
    selection.index == 0
}

fn element_info(element: VitalityElement) -> &'static str {
    match element {
        VitalityElement::Igna => "The element of fire. All about combustion, pressure waves, and relentless offense.",
        VitalityElement::Aqua => "Water disciplines focused on flow, adaptability, and battlefield control.",
        VitalityElement::Glacia => "Ice mages mix patience with brutal execution, freezing foes in place.",
        VitalityElement::Terra => "Earth wards thrive on endurance, fortifications, and seismic counters.",
        VitalityElement::Planta => "Life weaving. Regeneration, spores, and symbiotic buffs define this path.",
        VitalityElement::Venta => "Air channels speed, stances, and evasive counter-play with razor wind.",
        VitalityElement::Volt => "Lightning emphasizes burst damage, reflexes, and overclocked nerves.",
        VitalityElement::Umbra => "Void arts bend perception, fear, and shadow tether illusions.",
        VitalityElement::Null => "Unaligned focus. Keeps mana neutral until a mentor or relic imprints a path.",
    }
}
