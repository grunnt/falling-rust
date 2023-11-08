use std::fmt;

pub const ELEMENT_COUNT: usize = 27;

// The different element types that live in a cell in the sand box
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
    Gunpowder = 18,
    Fuse = 19,
    Explosion = 20,
    WaterSource = 21,
    AcidSource = 22,
    OilSource = 23,
    FireSource = 24,
    LavaSource = 25,
    Indestructible = 26,
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

// Definition of an element type
#[derive(Clone, Debug)]
pub struct ElementType {
    pub form: ElementForm,
    pub strength: u8,
    pub weight: u8,
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

// All element definitions. Note that the order must be identical to that in the Element enum.
pub static ELEMENTS: [ElementType; ELEMENT_COUNT] = [
    // Air = 0
    ElementType {
        form: ElementForm::Gas,
        strength: 1,
        weight: 128,
        color: (33, 122, 238),
        flags: FLAG_ALLOW_PLANT,
        source_element: Element::Air,
    }, //  Sand = 1,
    ElementType {
        form: ElementForm::Powder,
        strength: 8,
        weight: 1,
        color: (224, 198, 98),
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_NUTRITIOUS | FLAG_ALLOW_PLANT,
        source_element: Element::Air,
    }, // Rock = 2,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color: (107, 104, 104),
        flags: FLAG_BLAST_RESISTANT,
        source_element: Element::Air,
    }, // Water = 3,
    ElementType {
        form: ElementForm::Liquid,
        strength: 12,
        weight: 128,
        color: (16, 16, 128),
        flags: FLAG_CAUSES_RUST | FLAG_WET | FLAG_ALLOW_PLANT,
        source_element: Element::Air,
    }, // Acid = 4,
    ElementType {
        form: ElementForm::Liquid,
        strength: 10,
        weight: 32,
        color: (182, 255, 5),
        flags: FLAG_ACIDIC,
        source_element: Element::Air,
    }, // Drain = 5,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color: (0, 0, 0),
        flags: 0,
        source_element: Element::Air,
    }, // Wood = 6,
    ElementType {
        form: ElementForm::Solid,
        strength: 16,
        weight: 1,
        color: (122, 57, 0),
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_BURNS | FLAG_TURNS_INTO_ASH | FLAG_BLAST_RESISTANT,
        source_element: Element::Air,
    }, // Iron = 7,
    ElementType {
        form: ElementForm::Solid,
        strength: 64,
        weight: 1,
        color: (160, 157, 157),
        flags: FLAG_BLAST_RESISTANT,
        source_element: Element::Air,
    }, // Rust = 8,
    ElementType {
        form: ElementForm::Powder,
        strength: 1,
        weight: 1,
        color: (115, 50, 2),
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_CAUSES_RUST,
        source_element: Element::Air,
    }, // Fire = 9,
    ElementType {
        form: ElementForm::Gas,
        strength: 64,
        weight: 64,
        color: (255, 225, 136),
        flags: FLAG_IGNITES,
        source_element: Element::Air,
    }, // Ash = 10,
    ElementType {
        form: ElementForm::Powder,
        strength: 16,
        weight: 1,
        color: (214, 220, 234),
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_NUTRITIOUS | FLAG_ALLOW_PLANT,
        source_element: Element::Air,
    }, // Oil = 11,
    ElementType {
        form: ElementForm::Liquid,
        strength: 10,
        weight: 64,
        color: (64, 32, 64),
        flags: FLAG_BURNS,
        source_element: Element::Air,
    }, // Lava = 12,
    ElementType {
        form: ElementForm::Liquid,
        strength: 4,
        weight: 192,
        color: (180, 64, 16),
        flags: FLAG_IGNITES,
        source_element: Element::Air,
    }, // Smoke = 13,
    ElementType {
        form: ElementForm::Gas,
        strength: 32,
        weight: 32,
        color: (8, 8, 8),
        flags: 0,
        source_element: Element::Air,
    }, // Life = 14,
    ElementType {
        form: ElementForm::Solid,
        strength: 2,
        weight: 1,
        color: (210, 255, 210),
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_BURNS | FLAG_TURNS_INTO_ASH,
        source_element: Element::Air,
    }, // Seed = 15,
    ElementType {
        form: ElementForm::Powder,
        strength: 32,
        weight: 1,
        color: (170, 220, 130),
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_NUTRITIOUS,
        source_element: Element::Air,
    }, // Plant = 16,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color: (60, 200, 30),
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_BURNS | FLAG_NUTRITIOUS,
        source_element: Element::Air,
    }, // TNT = 17,
    ElementType {
        form: ElementForm::Solid,
        strength: 3,
        weight: 1,
        color: (200, 32, 16),
        flags: FLAG_DISSOLVES_IN_ACID,
        source_element: Element::Air,
    }, // Gunpowder = 18,
    ElementType {
        form: ElementForm::Powder,
        strength: 2,
        weight: 2,
        color: (122, 21, 3),
        flags: FLAG_DISSOLVES_IN_ACID,
        source_element: Element::Air,
    }, // Fuse = 19,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color: (211, 80, 91),
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_BURNS,
        source_element: Element::Air,
    }, // Explosion = 20,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color: (245, 220, 200),
        flags: 0,
        source_element: Element::Air,
    }, // WaterSource = 21,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color: (16, 16, 255),
        flags: FLAG_IS_SOURCE,
        source_element: Element::Water,
    }, // AcidSource = 22
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color: (160, 255, 64),
        flags: FLAG_IS_SOURCE,
        source_element: Element::Acid,
    }, // OilSource = 23,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color: (32, 8, 32),
        flags: FLAG_IS_SOURCE,
        source_element: Element::Oil,
    }, // FireSource = 24,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color: (255, 255, 163),
        flags: FLAG_IS_SOURCE | FLAG_IGNITES,
        source_element: Element::Fire,
    }, // LavaSource = 25,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color: (255, 128, 32),
        flags: FLAG_IS_SOURCE,
        source_element: Element::Lava,
    }, // Indestructible = 26,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
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
