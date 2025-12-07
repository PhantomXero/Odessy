use el_roi::{read_float, read_int};
use serde::{Deserialize, Serialize};
use std::fmt;

const HEIGHT_MIN: f64 = 150.0;
const HEIGHT_MAX: f64 = 200.0;
const WEIGHT_MIN: f64 = 50.0;
const WEIGHT_MAX: f64 = 150.0;

enum List {
    Build,
    Hand,
    Height,
    Weight,
    Skin,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum DominantHand {
    Right,
    Left,
    Ambidextrous,
}

impl fmt::Display for DominantHand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            DominantHand::Right => "Right",
            DominantHand::Left => "Left",
            DominantHand::Ambidextrous => "Ambidextrous",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum SkinColour {
    Black,
    White,
    Yellow,
    Red,
}

impl fmt::Display for SkinColour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            SkinColour::Black => "Black",
            SkinColour::White => "White",
            SkinColour::Yellow => "Yellow",
            SkinColour::Red => "Red",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Physic {
    Athletic,
    Lean,
    Muscular,
}

impl fmt::Display for Physic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Physic::Athletic => "Athletic",
            Physic::Lean => "Lean",
            Physic::Muscular => "Muscular",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PhysicalInfo {
    height: f64,
    weight: f64,
    physic: Physic,
    skin: SkinColour,
    dominant_hand: DominantHand,
}

impl PhysicalInfo {
    pub fn new() -> Self {
        let height = 179.0;
        let weight = 65.0;
        let physic = Physic::Lean;
        let skin = SkinColour::Black;
        let dominant_hand = DominantHand::Right;

        Self {
            height,
            weight,
            physic,
            skin,
            dominant_hand,
        }
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }

    pub fn physique(&self) -> Physic {
        self.physic
    }

    pub fn bmi(&self) -> f64 {
        let meters = self.height / 100.0;
        if meters == 0.0 {
            0.0
        } else {
            self.weight / (meters * meters)
        }
    }

    pub fn edit(&mut self) -> Result<(), String> {
        Self::list(List::Height);
        let height_res = read_float(&format!(
            "Enter your Height({HEIGHT_MIN}-{HEIGHT_MAX} cm): "
        ));
        Self::validate_height(height_res)?;
        self.height = height_res;

        Self::list(List::Weight);
        let weight_res = read_float(&format!(
            "Enter your Weight({WEIGHT_MIN}-{WEIGHT_MAX} kg): "
        ));
        Self::validate_weight(weight_res)?;
        self.weight = weight_res;

        Self::list(List::Build);
        self.physic = match read_int("Enter the number of your Physic: ") {
            1 => Physic::Athletic,
            2 => Physic::Lean,
            3 => Physic::Muscular,
            _ => Physic::Lean,
        };

        Self::list(List::Skin);
        self.skin = match read_int("Enter the number of your Skin Colour: ") {
            1 => SkinColour::Black,
            2 => SkinColour::Red,
            3 => SkinColour::White,
            4 => SkinColour::Yellow,
            _ => SkinColour::Black,
        };

        Self::list(List::Hand);
        self.dominant_hand = match read_int("Enter the number of your Dominant Hand: ") {
            1 => DominantHand::Ambidextrous,
            2 => DominantHand::Left,
            3 => DominantHand::Right,
            _ => DominantHand::Right,
        };

        Ok(())
    }

    pub fn level_up(&mut self) {
        self.physic = match self.physic {
            Physic::Lean => Physic::Athletic,
            Physic::Athletic => Physic::Muscular,
            Physic::Muscular => Physic::Muscular,
        };
    }

    pub fn list(list: List) {
        match list {
            List::Build => {
                println!("Physic");
                println!("1. {}", Physic::Athletic);
                println!("2. {}", Physic::Lean);
                println!("3. {}", Physic::Muscular);
            }
            List::Hand => {
                println!("Dominant Hand");
                println!("1. {}", DominantHand::Ambidextrous);
                println!("2. {}", DominantHand::Left);
                println!("3. {}", DominantHand::Right);
            }
            List::Height => {
                println!("Height Range: {HEIGHT_MIN}-{HEIGHT_MAX} cm");
            }
            List::Weight => {
                println!("Weight Range: {WEIGHT_MIN}-{WEIGHT_MAX} kg");
            }
            List::Skin => {
                println!("Skin Colour");
                println!("1. {}", SkinColour::Black);
                println!("2. {}", SkinColour::Red);
                println!("3. {}", SkinColour::White);
                println!("4. {}", SkinColour::Yellow);
            }
        }
    }

    fn validate_height(value: f64) -> Result<(), String> {
        if (HEIGHT_MIN..=HEIGHT_MAX).contains(&value) {
            Ok(())
        } else {
            Err(format!(
                "Height must be within {HEIGHT_MIN}-{HEIGHT_MAX} cm"
            ))
        }
    }

    fn validate_weight(value: f64) -> Result<(), String> {
        if (WEIGHT_MIN..=WEIGHT_MAX).contains(&value) {
            Ok(())
        } else {
            Err(format!(
                "Weight must be within {WEIGHT_MIN}-{WEIGHT_MAX} kg"
            ))
        }
    }
}

impl Default for PhysicalInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PhysicalInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Height : {:.1} cm", self.height)?;
        writeln!(f, "Weight : {:.1} kg", self.weight)?;
        writeln!(f, "Build  : {}", self.physic)?;
        writeln!(f, "Skin   : {}", self.skin)?;
        write!(f, "Hand   : {}", self.dominant_hand)
    }
}
