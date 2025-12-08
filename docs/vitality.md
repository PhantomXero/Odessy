# Vitality Crate Notes

The `vitality` crate owns elemental attunement, stamina pools, and spell scaffolding. This document will grow with balance tables and status effect specs.

## Outline
- **Elements & Levels** – `core.rs` exports `VitalityElement`, `VitalityLevel`, and the neutral `VitalityProfile`. Attributes stay low (control/power/cooldown rate) so gameplay loops can grow them alongside physical stats.
- **Mana Module** – `mana.rs` defines `ManaArchetype` + `ManaPool`, including the caster +10 capacity buffer and helpers for synchronizing archetypes when classes change.
- **Spell Scaffolding** – `spell.rs` owns `SpellSignature` (element tag, mana cost, cooldown) and lightweight `SpellBook` collections so any crate can register spells without duplicating math.
- **Progression Hooks** – `VitalityProfile::advance_level` is the single entry point for bumping ranks; it auto-applies modest stat bumps and leaves room for future training events.
- **Integration Points** – consumer crates (e.g., `character`) import the serde-friendly structs directly, keeping UI/state code thin while all elemental logic lives here.
