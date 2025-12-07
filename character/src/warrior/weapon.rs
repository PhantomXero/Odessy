use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Weapon {
    LongSword,
    ShortSword,
    Claymore,
    Saber,
    DualLongSword,
    DualShortSword,
    TwinDaggers,
    Axe,
    Hammer,
    WarMaul,
    TowerShield,
    BulwarkShield,
    Hands,
    Dagger,
    Whip,
    Gauntlets,
    ChannelingStaff,
    FocusingWand,
    Spellbook,
    ArcaneOrb,
    ShortBow,
    LongBow,
    Crossbow,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WeaponCategory {
    Swordsman,
    DualBlade,
    Tank,
    Caster,
    Ranger,
    Brawler,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WeaponWeight {
    Light,
    Medium,
    Heavy,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WeaponQuality {
    Crude,
    Standard,
    Superior,
    Masterwork,
}

impl Default for WeaponQuality {
    fn default() -> Self {
        WeaponQuality::Standard
    }
}

impl WeaponQuality {
    pub fn durability_bonus(self) -> f64 {
        match self {
            WeaponQuality::Crude => -8.0,
            WeaponQuality::Standard => 0.0,
            WeaponQuality::Superior => 12.0,
            WeaponQuality::Masterwork => 20.0,
        }
    }

    pub fn damage_bonus(self) -> f64 {
        match self {
            WeaponQuality::Crude => -2.0,
            WeaponQuality::Standard => 0.0,
            WeaponQuality::Superior => 3.0,
            WeaponQuality::Masterwork => 5.0,
        }
    }
}

impl Weapon {
    pub fn weight(&self) -> WeaponWeight {
        match self {
            Weapon::Hands
            | Weapon::Dagger
            | Weapon::Whip
            | Weapon::FocusingWand
            | Weapon::Spellbook
            | Weapon::ArcaneOrb
            | Weapon::TwinDaggers => WeaponWeight::Light,
            Weapon::ShortSword
            | Weapon::Saber
            | Weapon::DualShortSword
            | Weapon::Gauntlets
            | Weapon::ChannelingStaff
            | Weapon::ShortBow
            | Weapon::LongBow
            | Weapon::Crossbow => WeaponWeight::Medium,
            Weapon::LongSword
            | Weapon::Claymore
            | Weapon::DualLongSword
            | Weapon::Axe
            | Weapon::Hammer
            | Weapon::WarMaul
            | Weapon::TowerShield
            | Weapon::BulwarkShield => WeaponWeight::Heavy,
        }
    }

    pub fn is_prestige(&self) -> bool {
        matches!(
            self,
            Weapon::DualLongSword
                | Weapon::Hammer
                | Weapon::Claymore
                | Weapon::WarMaul
                | Weapon::BulwarkShield
                | Weapon::ArcaneOrb
                | Weapon::LongBow
        )
    }

    pub fn base_damage(&self) -> f64 {
        match self {
            Weapon::Hands => 8.0,
            Weapon::Dagger => 14.0,
            Weapon::Whip => 11.0,
            Weapon::Gauntlets => 18.0,
            Weapon::ShortSword => 20.0,
            Weapon::Saber => 22.0,
            Weapon::DualShortSword => 24.0,
            Weapon::TwinDaggers => 19.0,
            Weapon::LongSword => 30.0,
            Weapon::Claymore => 32.0,
            Weapon::DualLongSword => 34.0,
            Weapon::Axe => 32.0,
            Weapon::Hammer => 34.0,
            Weapon::WarMaul => 36.0,
            Weapon::TowerShield => 16.0,
            Weapon::BulwarkShield => 20.0,
            Weapon::ChannelingStaff => 18.0,
            Weapon::FocusingWand => 14.0,
            Weapon::Spellbook => 12.0,
            Weapon::ArcaneOrb => 20.0,
            Weapon::ShortBow => 24.0,
            Weapon::LongBow => 28.0,
            Weapon::Crossbow => 30.0,
        }
    }

    pub fn base_durability(&self) -> f64 {
        match self.weight() {
            WeaponWeight::Light => 22.0,
            WeaponWeight::Medium => 24.0,
            WeaponWeight::Heavy => 26.0,
        }
    }

    pub fn category(&self) -> WeaponCategory {
        match self {
            Weapon::LongSword | Weapon::ShortSword | Weapon::Claymore | Weapon::Saber => {
                WeaponCategory::Swordsman
            }
            Weapon::DualLongSword | Weapon::DualShortSword | Weapon::TwinDaggers => {
                WeaponCategory::DualBlade
            }
            Weapon::Axe
            | Weapon::Hammer
            | Weapon::WarMaul
            | Weapon::TowerShield
            | Weapon::BulwarkShield => WeaponCategory::Tank,
            Weapon::ChannelingStaff
            | Weapon::FocusingWand
            | Weapon::Spellbook
            | Weapon::ArcaneOrb => WeaponCategory::Caster,
            Weapon::ShortBow | Weapon::LongBow | Weapon::Crossbow => WeaponCategory::Ranger,
            Weapon::Hands | Weapon::Gauntlets | Weapon::Dagger | Weapon::Whip => {
                WeaponCategory::Brawler
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponInfo {
    class: Weapon,
    #[serde(default)]
    quality: WeaponQuality,
    damage: f64,
    durability: f64,
}

impl WeaponInfo {
    pub fn new() -> Self {
        Self::from_weapon(Weapon::Hands)
    }

    pub fn from_weapon(class: Weapon) -> Self {
        Self::from_weapon_with_quality(class, WeaponQuality::default())
    }

    pub fn from_weapon_with_quality(class: Weapon, quality: WeaponQuality) -> Self {
        let mut info = Self {
            class,
            quality,
            damage: 0.0,
            durability: 0.0,
        };
        info.refresh_stats();
        info
    }

    pub fn from_crude_loot(class: Weapon) -> Self {
        Self::from_weapon_with_quality(class, WeaponQuality::Crude)
    }

    pub fn try_reforge_crude(&mut self) -> WeaponQuality {
        if self.quality != WeaponQuality::Crude {
            println!("Only crude weapons can be reforged via salvage.");
            return self.quality;
        }
        let mut rng = rand::thread_rng();
        let roll: u16 = rng.gen_range(0..1000);
        self.quality = if roll == 0 {
            println!("Incredible craftsmanship! The crude weapon ascends to Masterwork.");
            WeaponQuality::Masterwork
        } else if roll < 10 {
            println!("Lucky strike! The weapon solidifies into Superior quality.");
            WeaponQuality::Superior
        } else if roll < 210 {
            println!("Salvage succeeded. Weapon restored to Standard quality.");
            WeaponQuality::Standard
        } else {
            println!("The reforge failed. The weapon remains crude.");
            WeaponQuality::Crude
        };
        self.refresh_stats();
        self.quality
    }

    pub fn new_weapons() -> Vec<Weapon> {
        vec![
            Weapon::Hands,
            Weapon::Gauntlets,
            Weapon::Dagger,
            Weapon::Whip,
            Weapon::ShortSword,
            Weapon::Saber,
            Weapon::DualShortSword,
            Weapon::LongSword,
            Weapon::Claymore,
            Weapon::DualLongSword,
            Weapon::Axe,
            Weapon::Hammer,
            Weapon::WarMaul,
            Weapon::TowerShield,
            Weapon::BulwarkShield,
            Weapon::TwinDaggers,
            Weapon::ChannelingStaff,
            Weapon::FocusingWand,
            Weapon::Spellbook,
            Weapon::ArcaneOrb,
            Weapon::ShortBow,
            Weapon::LongBow,
            Weapon::Crossbow,
        ]
    }

    pub fn class(&self) -> &Weapon {
        &self.class
    }

    pub fn quality(&self) -> WeaponQuality {
        self.quality
    }

    #[allow(dead_code)]
    pub fn damage(&self) -> f64 {
        self.damage
    }

    #[allow(dead_code)]
    pub fn durability(&self) -> f64 {
        self.durability
    }
}

impl WeaponInfo {
    fn refresh_stats(&mut self) {
        self.damage = if self.quality == WeaponQuality::Crude {
            1.0
        } else {
            (self.class.base_damage() + self.quality.damage_bonus()).max(1.0)
        };
        self.durability = (self.class.base_durability() + self.quality.durability_bonus()).max(5.0);
    }
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Weapon::LongSword => "Long Sword",
            Weapon::ShortSword => "Short Sword",
            Weapon::Claymore => "Claymore",
            Weapon::Saber => "Saber",
            Weapon::DualLongSword => "Dual Long Sword",
            Weapon::DualShortSword => "Dual Short Sword",
            Weapon::TwinDaggers => "Twin Daggers",
            Weapon::Axe => "Axe",
            Weapon::Hammer => "Hammer",
            Weapon::WarMaul => "War Maul",
            Weapon::TowerShield => "Tower Shield",
            Weapon::BulwarkShield => "Bulwark Shield",
            Weapon::Hands => "Hands",
            Weapon::Dagger => "Dagger",
            Weapon::Whip => "Whip",
            Weapon::Gauntlets => "Gauntlets",
            Weapon::ChannelingStaff => "Channeling Staff",
            Weapon::FocusingWand => "Focusing Wand",
            Weapon::Spellbook => "Spellbook",
            Weapon::ArcaneOrb => "Arcane Orb",
            Weapon::ShortBow => "Short Bow",
            Weapon::LongBow => "Long Bow",
            Weapon::Crossbow => "Crossbow",
        };
        write!(f, "{}", label)
    }
}

impl fmt::Display for WeaponCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            WeaponCategory::Swordsman => "Swordsman",
            WeaponCategory::DualBlade => "Dual Blade",
            WeaponCategory::Tank => "Tank",
            WeaponCategory::Caster => "Caster",
            WeaponCategory::Ranger => "Ranger",
            WeaponCategory::Brawler => "Brawler",
        };
        write!(f, "{}", label)
    }
}

impl fmt::Display for WeaponInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{}] (Damage {:.1}, Durability {:.1})",
            self.class, self.quality, self.damage, self.durability
        )
    }
}

impl fmt::Display for WeaponQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            WeaponQuality::Crude => "Crude",
            WeaponQuality::Standard => "Standard",
            WeaponQuality::Superior => "Superior",
            WeaponQuality::Masterwork => "Masterwork",
        };
        write!(f, "{}", label)
    }
}
