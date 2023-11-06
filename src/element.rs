use std::fmt;

pub const ELEMENT_COUNT: usize = 26;

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
    Fuse = 18,
    Explosion = 19,
    WaterSource = 20,
    AcidSource = 21,
    OilSource = 22,
    FireSource = 23,
    LavaSource = 24,
    Indestructible = 25,
}

pub fn element_name(element: Element) -> &'static str {
    match element {
        Element::Air => "Air",
        Element::Sand => "Sand",
        Element::Rock => "Rock",
        Element::Water => "Water",
        Element::Acid => "Acid",
        Element::Drain => "Drain",
        Element::Wood => "Wood",
        Element::Iron => "Iron",
        Element::Rust => "Rust",
        Element::Fire => "Fire",
        Element::Ash => "Ash",
        Element::Oil => "Oil",
        Element::Lava => "Lava",
        Element::Smoke => "Smoke",
        Element::Life => "Life",
        Element::Seed => "Seed",
        Element::Plant => "Plant",
        Element::TNT => "TNT",
        Element::Fuse => "Fuse",
        Element::Explosion => "Explosion",
        Element::WaterSource => "Water source",
        Element::AcidSource => "Acid source",
        Element::OilSource => "Oil source",
        Element::FireSource => "Fire source",
        Element::LavaSource => "Lava source",
        Element::Indestructible => "Indestructible",
    }
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
    pub color_1: (u8, u8, u8),
    pub color_2: (u8, u8, u8),
    pub render: RenderMethod,
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
        color_1: (33, 122, 238),
        color_2: (0, 0, 0),
        render: RenderMethod::FixedColor,
        flags: FLAG_ALLOW_PLANT,
        source_element: Element::Air,
    }, //  Sand = 1,
    ElementType {
        form: ElementForm::Powder,
        strength: 8,
        weight: 1,
        color_1: (229, 184, 125),
        color_2: (156, 97, 41),
        render: RenderMethod::VariantLinear,
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_NUTRITIOUS | FLAG_ALLOW_PLANT,
        source_element: Element::Air,
    }, // Rock = 2,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color_1: (178, 178, 178),
        color_2: (63, 63, 63),
        render: RenderMethod::VariantLinear,
        flags: FLAG_BLAST_RESISTANT,
        source_element: Element::Air,
    }, // Water = 3,
    ElementType {
        form: ElementForm::Liquid,
        strength: 12,
        weight: 128,
        color_1: (16, 16, 128),
        color_2: (12, 12, 100),
        render: RenderMethod::VariantLinear,
        flags: FLAG_CAUSES_RUST | FLAG_WET | FLAG_ALLOW_PLANT,
        source_element: Element::Air,
    }, // Acid = 4,
    ElementType {
        form: ElementForm::Liquid,
        strength: 10,
        weight: 32,
        color_1: (182, 255, 5),
        color_2: (5, 255, 40),
        render: RenderMethod::VariantLinear,
        flags: FLAG_ACIDIC,
        source_element: Element::Air,
    }, // Drain = 5,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color_1: (0, 0, 0),
        color_2: (0, 0, 0),
        render: RenderMethod::FixedColor,
        flags: 0,
        source_element: Element::Air,
    }, // Wood = 6,
    ElementType {
        form: ElementForm::Solid,
        strength: 16,
        weight: 1,
        color_1: (176, 110, 56),
        color_2: (73, 38, 22),
        render: RenderMethod::VariantLinear,
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_BURNS | FLAG_TURNS_INTO_ASH | FLAG_BLAST_RESISTANT,
        source_element: Element::Air,
    }, // Iron = 7,
    ElementType {
        form: ElementForm::Solid,
        strength: 64,
        weight: 1,
        color_1: (152, 148, 139),
        color_2: (100, 100, 90),
        render: RenderMethod::VariantLinear,
        flags: FLAG_BLAST_RESISTANT,
        source_element: Element::Air,
    }, // Rust = 8,
    ElementType {
        form: ElementForm::Powder,
        strength: 1,
        weight: 1,
        color_1: (115, 50, 2),
        color_2: (60, 40, 2),
        render: RenderMethod::VariantLinear,
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_CAUSES_RUST,
        source_element: Element::Air,
    }, // Fire = 9,
    ElementType {
        form: ElementForm::Gas,
        strength: 64,
        weight: 64,
        color_1: (255, 255, 159),
        color_2: (158, 61, 18),
        render: RenderMethod::Flicker,
        flags: FLAG_IGNITES,
        source_element: Element::Air,
    }, // Ash = 10,
    ElementType {
        form: ElementForm::Powder,
        strength: 16,
        weight: 1,
        color_1: (214, 220, 234),
        color_2: (124, 124, 136),
        render: RenderMethod::StrengthLinear,
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_NUTRITIOUS | FLAG_ALLOW_PLANT,
        source_element: Element::Air,
    }, // Oil = 11,
    ElementType {
        form: ElementForm::Liquid,
        strength: 10,
        weight: 64,
        color_1: (64, 32, 64),
        color_2: (32, 16, 32),
        render: RenderMethod::VariantLinear,
        flags: FLAG_BURNS,
        source_element: Element::Air,
    }, // Lava = 12,
    ElementType {
        form: ElementForm::Liquid,
        strength: 4,
        weight: 192,
        color_1: (180, 64, 16),
        color_2: (90, 32, 8),
        render: RenderMethod::VariantLinear,
        flags: FLAG_IGNITES,
        source_element: Element::Air,
    }, // Smoke = 13,
    ElementType {
        form: ElementForm::Gas,
        strength: 32,
        weight: 32,
        color_1: (8, 8, 8),
        color_2: (33, 122, 238),
        render: RenderMethod::StrengthLinear,
        flags: 0,
        source_element: Element::Air,
    }, // Life = 14,
    ElementType {
        form: ElementForm::Solid,
        strength: 2,
        weight: 1,
        color_1: (210, 255, 210),
        color_2: (105, 128, 105),
        render: RenderMethod::StrengthLinear,
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_BURNS | FLAG_TURNS_INTO_ASH,
        source_element: Element::Air,
    }, // Seed = 15,
    ElementType {
        form: ElementForm::Powder,
        strength: 32,
        weight: 1,
        color_1: (170, 220, 130),
        color_2: (0, 0, 0),
        render: RenderMethod::FixedColor,
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_NUTRITIOUS,
        source_element: Element::Air,
    }, // Plant = 16,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color_1: (60, 200, 30),
        color_2: (10, 30, 5),
        render: RenderMethod::VariantLinear,
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_BURNS | FLAG_NUTRITIOUS,
        source_element: Element::Air,
    }, // TNT = 17,
    ElementType {
        form: ElementForm::Solid,
        strength: 8,
        weight: 1,
        color_1: (200, 32, 16),
        color_2: (180, 24, 8),
        render: RenderMethod::VariantLinear,
        flags: FLAG_DISSOLVES_IN_ACID,
        source_element: Element::Air,
    }, // Fuse = 18,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color_1: (127, 51, 0),
        color_2: (0, 0, 0),
        render: RenderMethod::FixedColor,
        flags: FLAG_DISSOLVES_IN_ACID | FLAG_BURNS,
        source_element: Element::Air,
    }, // Explosion = 19,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color_1: (245, 220, 200),
        color_2: (255, 255, 255),
        render: RenderMethod::Flicker,
        flags: 0,
        source_element: Element::Air,
    }, // WaterSource = 20,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color_1: (16, 16, 255),
        color_2: (0, 0, 0),
        render: RenderMethod::FixedColor,
        flags: FLAG_IS_SOURCE,
        source_element: Element::Water,
    }, // AcidSource = 21,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color_1: (160, 255, 64),
        color_2: (0, 0, 0),
        render: RenderMethod::FixedColor,
        flags: FLAG_IS_SOURCE,
        source_element: Element::Acid,
    }, // OilSource = 22,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color_1: (32, 8, 32),
        color_2: (0, 0, 0),
        render: RenderMethod::FixedColor,
        flags: FLAG_IS_SOURCE,
        source_element: Element::Oil,
    }, // FireSource = 23,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color_1: (255, 255, 16),
        color_2: (0, 0, 0),
        render: RenderMethod::FixedColor,
        flags: FLAG_IS_SOURCE | FLAG_IGNITES,
        source_element: Element::Fire,
    }, // LavaSource = 24,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color_1: (255, 128, 32),
        color_2: (0, 0, 0),
        render: RenderMethod::FixedColor,
        flags: FLAG_IS_SOURCE,
        source_element: Element::Lava,
    }, // Indestructible = 25,
    ElementType {
        form: ElementForm::Solid,
        strength: 1,
        weight: 1,
        color_1: (64, 40, 40),
        color_2: (0, 0, 0),
        render: RenderMethod::FixedColor,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderMethod {
    FixedColor,
    StrengthLinear,
    VariantLinear,
    Flicker,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
