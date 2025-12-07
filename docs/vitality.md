# Vitality Crate Notes

The `vitality` crate owns elemental attunement, stamina pools, and spell scaffolding. This document will grow with balance tables and status effect specs.

## Outline
- **Elements & Levels** – catalog enums such as `VitalityElement` and `VitalityLevel`, including the one-element-only rule.
- **Spells & Status** – placeholder for spell cards, cooldown math, and how vitality interacts with physical stats.
- **Progression Hooks** – describe how `VitalityInfo::level_up` or future training loops feed into the global character level.
- **Integration Points** – where the crate exposes serde, how it syncs with storage/UI, and how other crates should request vitality changes.

_Details to be added alongside the spell system implementation._
