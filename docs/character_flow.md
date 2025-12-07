# Character System Notes

## Non-Combat NPCs
- `Character` stores `warrior: Option<WarriorInfo>`. Civilians keep this field as `None`, which avoids empty or meaningless combat stats.
- During creation we ask "Is this character a combatant?" and only instantiate `WarriorInfo` when the answer is yes.
- NPC templates created for scripted events can call `Character::with_components(..., None)` to guarantee they never load combat data.
- When `WarriorInfo::from_prompt` arrives later, the `Character::from_prompt` flow already routes combatants through `WarriorInfo::from_prompt(&physical)` while civilians stay null.

## Creation Flow
1. **Identity** – `PersonalInfo::from_prompt()` gathers first name, last name, nickname/call sign, age, gender (with on-demand edits via `edit_identity_section`).
2. **History** – `CivicInfo::edit()` prints lore guidance and captures nationality + social class. Prestige-based level-ups happen through `CivicInfo::level_up()`.
3. **Physical Profile** – `PhysicalInfo::edit()` enforces height/weight ranges, suggests a build from BMI, and recalculates speed/strength modifiers that gate future weapon choices.
4. **Vitality** – `VitalityInfo::from_prompt()` reiterates that only one element may ever be chosen, then stores the attunement and level track.
5. **Combat (optional)** – Civilians skip this. Combatants run through `WarriorInfo::from_prompt(&physical)` to pick a class, style, and legal weapon based on their body stats.

## Physical Logic
- Height and weight feed a BMI recommendation that maps to `Physic::Lean | Athletic | Muscular`.
- Players can override the recommendation, but `can_apply_physic` blocks illogical combos (e.g., BMI 18 can't be Muscular).
- Speed and strength now share a 10-point pool at level 1 that grows toward 100 via physical rank-ups, combat XP, or scripted events. Each physique enforces minimums and caps so body types feel distinct.
- `recalculate_stats()` keeps the pool balanced after BMI/height/weight modifiers, applies temporary buffs, and exposes 1–100 values that weapon gating can read.
- `level_up()` now checks physique-specific requirements before awarding more points, adds 5 auto-allocation points per rank, then nudges the build toward its heavier counterpart.
- Combined speed + strength place the hero in a named tier (Drifter → Strider → Vanguard → Titan → Colossus) that now shows up on the physical card for quick reference.
- Dominant-hand selection is locked to left/right. Each hand tracks control/power ratings (stored on the profile) so dual wielders can feel different per side; training events can slowly push the off-hand toward ambidexterity.

## Combat Roles & Constraints
- Warrior classes now use the familiar Support/Tank/Healer/Vanguard/Brawler vocabulary.
- `WarriorInfo::from_prompt` walks players through class + optional sub-class, fighting style, and a weapon choice.
- Medium weapon access now keys off physique: athletic builds sit squarely in the sweet spot while lean or muscular frames must meet stricter ranges. Heavy gear is limited to muscular heroes (or Titan-tier athletics), and prestige weapons add an explicit Titan-tier gate.
- Weapon creation defaults to Standard quality. Upgrades (quality selection, class swaps, fighting-style respecs) are locked behind story-driven respec tokens so early choices stay meaningful. Crude gear is intentionally awful (damage = 1), only drops via botched loot/crafting tables, and requires salvage with a very slim (0.1%) chance to jump straight to Masterwork.
- `sync_with_physical` keeps existing warriors honest when body edits happen later.

## Storage & CLI UX
- Characters serialize to JSON and live inside `odessy_characters.db` via `rusqlite`. Each save row stores `id`, `name`, and a JSON blob.
- On startup, a Ratatui-powered selector lets you move through existing saves or create a new hero.
- The profile screen renders the multi-section card inside a bordered panel with action hints: level up, edit identity/history/physical/vitality/warrior, save, or quit.
- Each editing action reuses the same prompt logic the creation flow uses, so the CLI and eventual GUI share one source of truth.
