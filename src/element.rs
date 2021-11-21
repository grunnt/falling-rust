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
    Fire,
    Ash,
    Oil,
    Lava,
    Smoke,
    Life,
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
            Element::Sand => ElementForm::Powder,
            Element::Ash => ElementForm::Powder,
            _ => ElementForm::Solid,
        }
    }

    pub fn dissolves_in_acid(&self) -> bool {
        match self {
            Element::Sand => true,
            Element::Wood => true,
            Element::Ash => true,
            Element::Life => true,
            _ => false,
        }
    }

    pub fn burns(&self) -> bool {
        match self {
            Element::Wood => true,
            Element::Oil => true,
            Element::Life => true,
            _ => false,
        }
    }

    pub fn strength(&self) -> u8 {
        match self {
            Element::Fire => 16,
            Element::Acid => 32,
            Element::Wood => 4,
            Element::Oil => 1,
            Element::Lava => 64,
            Element::Smoke => 32,
            Element::Sand => 8,
            _ => 0,
        }
    }

    pub fn randomize_color_factor(&self) -> f32 {
        match self {
            Element::Sand => 0.25,
            Element::Water => 0.25,
            Element::Acid => 0.1,
            Element::Rock => 0.5,
            Element::Wood => 0.65,
            Element::Fire => 0.5,
            Element::Ash => 1.0,
            Element::Oil => 0.25,
            Element::Lava => 0.25,
            _ => 0.0,
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            Element::Air => (33, 122, 238),
            Element::Rock => (32, 32, 32),
            Element::Wood => (119, 64, 27),
            Element::Sand => (219, 176, 125),
            Element::Water => (8, 8, 128),
            Element::Drain => (0, 0, 0),
            Element::Acid => (80, 255, 32),
            Element::Fire => (255, 128, 8),
            Element::Ash => (16, 16, 16),
            Element::Oil => (16, 4, 16),
            Element::Lava => (128, 32, 8),
            Element::Life => (255, 255, 255),
            Element::Indestructible => (64, 40, 40),
            Element::Smoke => (16, 16, 16),
            Element::WaterSource => (8, 8, 128),
            Element::AcidSource => (80, 255, 32),
            Element::OilSource => (16, 4, 16),
            Element::FireSource => (255, 128, 8),
            Element::LavaSource => (128, 32, 8),
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
