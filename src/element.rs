use std::fmt;

pub const ELEMENT_COUNT: usize = 26;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Element {
    Air = 0,
    Sand = 1,
    Rock = 2,
    Water = 3,
    Acid = 4,
    Drain = 5,
    Wood = 6,
    Iron = 7,
    Rust = 8,
    Fire = 9,
    Ash = 10,
    Oil = 11,
    Lava = 12,
    Smoke = 13,
    Life = 14,
    Seed = 15,
    Plant = 16,
    TNT = 17,
    Fuse = 18,
    Explosion = 19,
    WaterSource = 20,
    AcidSource = 21,
    OilSource = 22,
    FireSource = 23,
    LavaSource = 24,
    Indestructible = 25,
}

pub const FLAG_DISSOLVES_IN_ACID: u32 = 0b00000000000000000000000000000001;
pub const FLAG_BURNS: u32 = 0b00000000000000000000000000000010;
pub const FLAG_CAUSES_RUST: u32 = 0b00000000000000000000000000000100;
pub const FLAG_TURNS_INTO_ASH: u32 = 0b00000000000000000000000000001000;
pub const FLAG_NUTRITIOUS: u32 = 0b00000000000000000000000000010000;
pub const FLAG_WET: u32 = 0b00000000000000000000000000100000;
pub const FLAG_ALLOW_PLANT: u32 = 0b00000000000000000000000001000000;
pub const FLAG_IS_SOURCE: u32 = 0b00000000000000000000000010000000;
pub const FLAG_IGNITES: u32 = 0b00000000000000000000000100000000;
pub const FLAG_BLAST_RESISTANT: u32 = 0b00000000000000000000001000000000;
pub const FLAG_ACIDIC: u32 = 0b00000000000000000000010000000000;
// TODO flag explodes

#[derive(Clone, Debug)]
pub struct ElementType {
    pub form: ElementForm,
    pub strength: u8,
    pub weight: u8,
    pub randomize_color_factor: f32, // TODO replace by color_1 and color_2
    pub color: (u8, u8, u8),
    pub flags: u32,
    pub source_element: Element,
}

impl ElementType {
    pub fn has_flag(&self, flag: u32) -> bool {
        self.flags & flag > 0
    }
}

#[inline(always)]
pub fn element_type(element: Element) -> &'static ElementType {
    &ELEMENTS[element as usize]
}

pub static ELEMENTS: [ElementType; ELEMENT_COUNT] = [
    // Air = 0
    ElementType {
        form: ElementForm::Gas,
        strength: 1,
        weight: 128,
        randomize_color_factor: 0.0,
        color: (33, 122, 238),
        flags: FLAG_ALLOW_PLANT,
        source_element: Element::Air,
    }, //  Sand = 1,
    ElementType {
        form: ElementForm::Powder,
        strength: 8,
        weight: 1,
        randomize_color_factor: 0.25,
        color: (219, 176, 125),
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_NUTRITIOUS | FLAG_ALLOW_PLANT,
        source_element: Element::Air,
    }, // Rock = 2,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        randomize_color_factor: 0.5,
        color: (128, 128, 128),
        flags: FLAG_BLAST_RESISTANT,
        source_element: Element::Air,
    }, // Water = 3,
    ElementType {
        form: ElementForm::Liquid,
        strength: 12,
        weight: 128,
        randomize_color_factor: 0.25,
        color: (16, 16, 128),
        flags: FLAG_CAUSES_RUST | FLAG_WET | FLAG_ALLOW_PLANT,
        source_element: Element::Air,
    }, // Acid = 4,
    ElementType {
        form: ElementForm::Liquid,
        strength: 10,
        weight: 32,
        randomize_color_factor: 0.1,
        color: (80, 255, 32),
        flags: FLAG_ACIDIC,
        source_element: Element::Air,
    }, // Drain = 5,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        randomize_color_factor: 0.0,
        color: (0, 0, 0),
        flags: 0,
        source_element: Element::Air,
    }, // Wood = 6,
    ElementType {
        form: ElementForm::Solid,
        strength: 16,
        weight: 1,
        randomize_color_factor: 0.4,
        color: (119, 64, 27),
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_BURNS | FLAG_TURNS_INTO_ASH | FLAG_BLAST_RESISTANT,
        source_element: Element::Air,
    }, // Iron = 7,
    ElementType {
        form: ElementForm::Solid,
        strength: 64,
        weight: 1,
        randomize_color_factor: 0.15,
        color: (151, 152, 157),
        flags: FLAG_BLAST_RESISTANT,
        source_element: Element::Air,
    }, // Rust = 8,
    ElementType {
        form: ElementForm::Powder,
        strength: 1,
        weight: 1,
        randomize_color_factor: 0.35,
        color: (184, 83, 46),
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_CAUSES_RUST,
        source_element: Element::Air,
    }, // Fire = 9,
    ElementType {
        form: ElementForm::Gas,
        strength: 64,
        weight: 64,
        randomize_color_factor: 0.5,
        color: (255, 180, 8),
        flags: FLAG_IGNITES,
        source_element: Element::Air,
    }, // Ash = 10,
    ElementType {
        form: ElementForm::Powder,
        strength: 16,
        weight: 1,
        randomize_color_factor: 0.9,
        color: (16, 16, 16),
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_NUTRITIOUS | FLAG_ALLOW_PLANT,
        source_element: Element::Air,
    }, // Oil = 11,
    ElementType {
        form: ElementForm::Liquid,
        strength: 10,
        weight: 64,
        randomize_color_factor: 0.25,
        color: (64, 32, 64),
        flags: FLAG_BURNS,
        source_element: Element::Air,
    }, // Lava = 12,
    ElementType {
        form: ElementForm::Liquid,
        strength: 4,
        weight: 192,
        randomize_color_factor: 0.25,
        color: (180, 64, 16),
        flags: FLAG_IGNITES,
        source_element: Element::Air,
    }, // Smoke = 13,
    ElementType {
        form: ElementForm::Gas,
        strength: 32,
        weight: 32,
        randomize_color_factor: 0.0,
        color: (9, 36, 68),
        flags: 0,
        source_element: Element::Air,
    }, // Life = 14,
    ElementType {
        form: ElementForm::Solid,
        strength: 2,
        weight: 1,
        randomize_color_factor: 0.1,
        color: (210, 255, 210),
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_BURNS | FLAG_TURNS_INTO_ASH,
        source_element: Element::Air,
    }, // Seed = 15,
    ElementType {
        form: ElementForm::Powder,
        strength: 32,
        weight: 1,
        randomize_color_factor: 0.1,
        color: (170, 220, 130),
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_NUTRITIOUS,
        source_element: Element::Air,
    }, // Plant = 16,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        randomize_color_factor: 0.7,
        color: (60, 200, 30),
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_BURNS | FLAG_NUTRITIOUS,
        source_element: Element::Air,
    }, // TNT = 17,
    ElementType {
        form: ElementForm::Solid,
        strength: 8,
        weight: 1,
        randomize_color_factor: 0.2,
        color: (200, 32, 16),
        flags: FLAG_DISSOLVES_IN_ACID,
        source_element: Element::Air,
    }, // Fuse = 18,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        randomize_color_factor: 0.0,
        color: (127, 51, 0),
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_BURNS,
        source_element: Element::Air,
    }, // Explosion = 19,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        randomize_color_factor: 0.1,
        color: (245, 220, 200),
        flags: 0,
        source_element: Element::Air,
    }, // WaterSource = 20,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        randomize_color_factor: 0.0,
        color: (16, 16, 255),
        flags: FLAG_IS_SOURCE,
        source_element: Element::Water,
    }, // AcidSource = 21,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        randomize_color_factor: 0.0,
        color: (160, 255, 64),
        flags: FLAG_IS_SOURCE,
        source_element: Element::Acid,
    }, // OilSource = 22,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        randomize_color_factor: 0.0,
        color: (32, 8, 32),
        flags: FLAG_IS_SOURCE,
        source_element: Element::Oil,
    }, // FireSource = 23,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        randomize_color_factor: 0.0,
        color: (255, 255, 16),
        flags: FLAG_IS_SOURCE | FLAG_IGNITES,
        source_element: Element::Fire,
    }, // LavaSource = 24,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        randomize_color_factor: 0.0,
        color: (255, 128, 32),
        flags: FLAG_IS_SOURCE,
        source_element: Element::Lava,
    }, // Indestructible = 25,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        randomize_color_factor: 0.0,
        color: (64, 40, 40),
        flags: 0,
        source_element: Element::Air,
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementForm {
    Solid,
    Powder,
    Liquid,
    Gas,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
