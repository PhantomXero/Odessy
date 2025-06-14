use el_roi::{read_float, read_int};

enum List {
    Build,
    Hand,
    Height,
    Weight,
    Skin,
}

#[derive(Debug)]
enum DominantHand {
    Right,
    Left,
    Ambidextrous,
}

#[derive(Debug)]
enum SkinColour {
    Black,
    White,
    Yellow,
    Red,
}

#[derive(Debug)]
enum Physic {
    Athletic,
    Lean,
    Muscular,
}

#[derive(Debug)]
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
    pub fn edit(&mut self) {
        Self::list(List::Height);
        let height_res = read_float("Enter your Height(150-200 cm): ");
        if height_res >= 150.0 && height_res <= 200.0 {
            self.height = height_res;
        } else {
            println!("Invalid Height");
            Self::list(List::Height);
            self.height = read_float("Enter your Height(150-200 cm): ");
        };
        Self::list(List::Weight);
        let weight_res = read_float("Enter your Weigh(50-150 kg)t: ");
        if weight_res >= 150.0 && weight_res <= 200.0 {
            self.weight = weight_res;
        } else {
            println!("Invalid weight");
            Self::list(List::Weight);
            self.weight = read_float("Enter your Weigh(50-150 kg)t: ");
        };
        Self::list(List::Build);
        let physic = match read_int("Enter the number of your Physic: ") {
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
        }
    }
    pub fn level_up(&mut self) {
        match self.physic {
            Physic::Lean => self.physic = Physic::Athletic,
            Physic::Muscular => self.physic = Physic::Muscular,
            _ => self.physic = Physic::Lean,
        }
    }
    pub fn list(list: List) {
        match list {
            List::Build => {
                println!("Physic");
                println!("1. {:?}", Physic::Athletic);
                println!("2. {:?}", Physic::Lean);
                println!("3. {:?}", Physic::Muscular);
            }
            List::Hand => {
                println!("Dominant Hand");
                println!("1. {:?}", DominantHand::Ambidextrous);
                println!("2. {:?}", DominantHand::Left);
                println!("3. {:?}", DominantHand::Right);
            }
            List::Height => {}
            List::Weight => {}
            List::Skin => {
                println!("Skin Colour");
                println!("1. {:?}", SkinColour::Black);
                println!("2. {:?}", SkinColour::Red);
                println!("3. {:?}", SkinColour::White);
                println!("4. {:?}", SkinColour::Yellow);
            }
            _ => {}
        }
    }
}
