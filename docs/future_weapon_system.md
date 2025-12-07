# Future Weapon System Plan

## Goals
- Broaden available weapons so each fighting style feels distinct (casters with staves/wands/tomes, tanks with shields/mauls, rangers with bows, etc.).
- Introduce a "range" dimension (Close, Mid, Long) alongside existing Light/Medium/Heavy weights so attributes can scale from both mass and distance.
- Keep attribute templates consistent within a weapon family (casters emphasize magic damage, focus cost, refresh time; tanks emphasize block value, stagger resistance, and swing power; ranged kits track draw speed, projectile stability, reload rhythm).
- Preserve current tier gating, quality tiers, and crude/standard/superior/masterwork logic while making it extensible for new archetypes.

## Proposed Weapon Families
### Casters
- **Examples:** Channeling Staff, Arcanist Wand, Grimoire, Catalyst Orb.
- **Stats:** Magic Damage, Focus Efficiency, Refresh Time, Channel Stability, Durability.
- **Range Defaults:** Mostly Mid (staff/orb) and Long (ritual grimoire) to keep them off the front line.
- **Notes:** Could consume or recharge Vitality resource, so integration points must exist between `WarriorInfo` and `VitalityInfo`.

### Tanks / Vanguards
- **Examples:** Tower Shield, Kite Shield + Mace, Maul, Poleaxe.
- **Stats:** Strike Power, Guard Value, Guard Break Resistance, Recovery Time, Durability.
- **Range Defaults:** Close; maybe a few Mid (pole weapons) to differentiate heavy reach options.
- **Notes:** Shields may need their own block meter separate from weapon durability.

### Brawlers / Skirmishers
- **Examples:** Claw Gauntlets, Hook Blades, Chain Whips, Tonfa.
- **Stats:** Combo Speed, Precision Window, Grapple Bonus, Durability.
- **Range Defaults:** Close/Mid split depending on leverage.
- **Notes:** These already exist partially (Hands, Gauntlets, Whip). We would refactor them to share templates with the other families.

### Rangers / Marksmen
- **Examples:** Short Bow, Long Bow, Recurve Bow, Crossbow, Throwing Chakram.
- **Stats:** Draw Strength (tied to Physical Strength), Nock/Reload Speed, Effective Range, Stability, Ammo Capacity.
- **Range Defaults:** Mid and Long.
- **Notes:** Introduces ammo/maintenance hooks. Need a lightweight way to store quiver/capacity without turning the CLI into inventory management immediately.

### Hybrid / Support
- **Examples:** Banner Lance, Signal Horn (buff device), Medic Sling.
- **Stats:** Buff Potency, Cooldown, Range, Durability.
- **Range Defaults:** Mid.
- **Notes:** Optional for later once base families land.

## Range Dimension
- Add `WeaponRange { Close, Mid, Long }` next to the existing `WeaponWeight` enum.
- Range influences default stats (Close promotes power/guard, Long boosts precision/reload). Could also tie into hit chances once combat math exists.
- Selection UI would show both weight and range (`Long Bow [Long | Medium]`).

## Data Model Changes (Future)
1. Expand `Weapon` enum or convert to struct-driven registry so each entry can declare:
   - Fighting style tags (Caster, Tank, Ranger, Hybrid).
   - Weight and range.
   - Attribute template (fields described above).
2. Update `WeaponInfo` to store new attributes plus optional ammo/focus pools.
3. Revisit `weapon_requirement` logic so certain range/weight combos demand specific physical tiers or dominant-hand profiles.
4. Extend serialization/migrations (DB payload) once the weapon struct changes.

## Rollout Strategy
1. **Doc+Design (this file).**
2. **Data Scaffolding:** introduce `WeaponRange` and update enums without surfacing new weapons yet.
3. **Caster Pass:** add staff/wand/grimoire weapons, map them to Support/Healer styles, ensure tier gates.
4. **Ranger Pass:** add bows/crossbows, implement range-aware stats, confirm UI renders them cleanly.
5. **Tank Pass:** add shielded kits, guard stats, and maybe dual-slot handling.
6. **Balance & QA:** tune numbers, integrate salvaging and tier upgrades for new categories.

## Complexity Notes
- The idea is intentionally staged; trying to land every weapon family plus range logic in one feature would be risky.
- Each pass should come with migration helpers (e.g., default existing saves to "Hands [Close | Light]").
- Combat math does not yet consume range/power stats, so we can treat them as flavorful metadata until the battle loop materializes.
