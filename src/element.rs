#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Element {
    Air,
    Sand,
    Rock,
    Water,
    Acid,
    Drain,
    Wood,
    Iron,
    Rust,
    Fire,
    Ash,
    Oil,
    Lava,
    Smoke,
    Life,
    Seed,
    Plant,
    TNT,
    Explosion,
    WaterSource,
    AcidSource,
    OilSource,
    FireSource,
    LavaSource,
    Indestructible,
}

impl Element {
    pub fn form(&self) -> ElementForm {
        match self {
            Element::Air => ElementForm::Gas,
            Element::Water => ElementForm::Liquid,
            Element::Acid => ElementForm::Liquid,
            Element::Oil => ElementForm::Liquid,
            Element::Lava => ElementForm::Liquid,
            Element::Fire => ElementForm::Gas,
            Element::Smoke => ElementForm::Gas,
            Element::Explosion => ElementForm::Gas,
            Element::Sand => ElementForm::Powder,
            Element::Ash => ElementForm::Powder,
            Element::Rust => ElementForm::Powder,
            Element::Seed => ElementForm::Powder,
            _ => ElementForm::Solid,
        }
    }

    pub fn dissolves_in_acid(&self) -> bool {
        match self {
            Element::Sand => true,
            Element::Wood => true,
            Element::Ash => true,
            Element::Life => true,
            Element::Rust => true,
            Element::Plant => true,
            Element::Seed => true,
            Element::TNT => true,
            _ => false,
        }
    }

    pub fn burns(&self) -> bool {
        match self {
            Element::Wood => true,
            Element::Oil => true,
            Element::Life => true,
            Element::Plant => true,
            _ => false,
        }
    }

    pub fn causes_rust(&self) -> bool {
        match self {
            Element::Water => true,
            Element::Rust => true,
            _ => false,
        }
    }

    pub fn plant_nutrition(&self) -> bool {
        match self {
            Element::Sand => true,
            Element::Ash => true,
            Element::Seed => true,
            Element::Plant => true,
            _ => false,
        }
    }

    pub fn plant_watering(&self) -> bool {
        match self {
            Element::Water => true,
            _ => false,
        }
    }

    pub fn allows_plant_growth(&self) -> bool {
        match self {
            Element::Air => true,
            Element::Sand => true,
            Element::Ash => true,
            Element::Water => true,
            _ => false,
        }
    }

    pub fn blast_resistance(&self) -> u8 {
        match self {
            Element::Rock => 16,
            Element::Iron => 12,
            Element::Wood => 4,
            Element::Sand => 4,
            _ => 1,
        }
    }

    pub fn strength(&self) -> u8 {
        match self {
            Element::Fire => 8,
            Element::Acid => 32,
            Element::Wood => 4,
            Element::Oil => 1,
            Element::Lava => 64,
            Element::Smoke => 32,
            Element::Sand => 8,
            Element::Iron => 64,
            Element::Seed => 32,
            Element::TNT => 4,
            _ => 0,
        }
    }

    pub fn randomize_color_factor(&self) -> f32 {
        match self {
            Element::Sand => 0.25,
            Element::Water => 0.25,
            Element::Acid => 0.1,
            Element::Rock => 0.5,
            Element::Wood => 0.4,
            Element::Fire => 0.5,
            Element::Ash => 1.0,
            Element::Oil => 0.25,
            Element::Lava => 0.25,
            Element::Iron => 0.15,
            Element::Rust => 0.35,
            Element::Plant => 0.7,
            Element::Seed => 0.1,
            Element::TNT => 0.2,
            Element::Explosion => 0.1,
            _ => 0.0,
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            Element::Air => (33, 122, 238),
            Element::Rock => (128, 128, 128),
            Element::Wood => (119, 64, 27),
            Element::Iron => (151, 152, 157),
            Element::Rust => (184, 83, 46),
            Element::Sand => (219, 176, 125),
            Element::Water => (16, 16, 128),
            Element::Drain => (0, 0, 0),
            Element::Acid => (80, 255, 32),
            Element::Fire => (255, 180, 8),
            Element::Ash => (16, 16, 16),
            Element::Oil => (64, 32, 64),
            Element::Lava => (160, 64, 32),
            Element::Life => (255, 255, 255),
            Element::Plant => (60, 200, 30),
            Element::Seed => (170, 220, 130),
            Element::Indestructible => (64, 40, 40),
            Element::Smoke => (9, 36, 68),
            Element::WaterSource => (8, 8, 128),
            Element::AcidSource => (80, 255, 32),
            Element::OilSource => (16, 4, 16),
            Element::FireSource => (255, 128, 8),
            Element::LavaSource => (128, 32, 8),
            Element::TNT => (200, 32, 16),
            Element::Explosion => (245, 220, 200),
        }
    }

    pub fn is_source(&self) -> bool {
        match self {
            Element::WaterSource => true,
            Element::AcidSource => true,
            Element::OilSource => true,
            Element::FireSource => true,
            Element::LavaSource => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementForm {
    Solid,
    Powder,
    Liquid,
    Gas,
}
