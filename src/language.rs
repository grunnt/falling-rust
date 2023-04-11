use std::collections::HashMap;

use crate::element::Element;

// Simplistic translation system to let my kids play more easily
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Language {
    English,
    Nederlands,
}

pub fn element_names(language: Language) -> HashMap<Element, String> {
    let mut names = HashMap::new();

    match language {
        Language::English => set_english_element_names(&mut names),
        Language::Nederlands => set_dutch_element_names(&mut names),
    }

    names
}

fn set_english_element_names(names: &mut HashMap<Element, String>) {
    names.insert(Element::Air, "Air".to_string());
    names.insert(Element::Acid, "Acid".to_string());
    names.insert(Element::AcidSource, "Acid source".to_string());
    names.insert(Element::Ash, "Ash".to_string());
    names.insert(Element::Drain, "Drain".to_string());
    names.insert(Element::Explosion, "Explision".to_string());
    names.insert(Element::Fire, "Fire".to_string());
    names.insert(Element::FireSource, "Fire source".to_string());
    names.insert(Element::Fuse, "Fuse".to_string());
    names.insert(Element::Indestructible, "Indestructible".to_string());
    names.insert(Element::Iron, "Iron".to_string());
    names.insert(Element::Lava, "Lava".to_string());
    names.insert(Element::LavaSource, "Lava source".to_string());
    names.insert(Element::Life, "Life".to_string());
    names.insert(Element::Oil, "Oil".to_string());
    names.insert(Element::OilSource, "Oil source".to_string());
    names.insert(Element::Plant, "Plant".to_string());
    names.insert(Element::Rock, "Stone".to_string());
    names.insert(Element::Rust, "Rust".to_string());
    names.insert(Element::Sand, "Sand".to_string());
    names.insert(Element::Seed, "Seed".to_string());
    names.insert(Element::Smoke, "Smoke".to_string());
    names.insert(Element::TNT, "TNT".to_string());
    names.insert(Element::Water, "Water".to_string());
    names.insert(Element::WaterSource, "Water source".to_string());
    names.insert(Element::Wood, "Wood".to_string());
}

fn set_dutch_element_names(names: &mut HashMap<Element, String>) {
    names.insert(Element::Air, "Luucht".to_string());
    names.insert(Element::Acid, "Zuur".to_string());
    names.insert(Element::AcidSource, "Zuur bron".to_string());
    names.insert(Element::Ash, "As".to_string());
    names.insert(Element::Drain, "Afvoer".to_string());
    names.insert(Element::Explosion, "Explosie".to_string());
    names.insert(Element::Fire, "Vuur".to_string());
    names.insert(Element::FireSource, "Vuuur bron".to_string());
    names.insert(Element::Fuse, "Lont".to_string());
    names.insert(Element::Indestructible, "Onkwetsbaar".to_string());
    names.insert(Element::Iron, "IJzer".to_string());
    names.insert(Element::Lava, "Lava".to_string());
    names.insert(Element::LavaSource, "Lava bron".to_string());
    names.insert(Element::Life, "Leven".to_string());
    names.insert(Element::Oil, "Olie".to_string());
    names.insert(Element::OilSource, "Olie bron".to_string());
    names.insert(Element::Plant, "Plant".to_string());
    names.insert(Element::Rock, "Steen".to_string());
    names.insert(Element::Rust, "Roest".to_string());
    names.insert(Element::Sand, "Zand".to_string());
    names.insert(Element::Seed, "Zaad".to_string());
    names.insert(Element::Smoke, "Rook".to_string());
    names.insert(Element::TNT, "TNT".to_string());
    names.insert(Element::Water, "Water".to_string());
    names.insert(Element::WaterSource, "Water bron".to_string());
    names.insert(Element::Wood, "Hout".to_string());
}

pub fn get_text(code: &str, language: Language) -> &str {
    match language {
        Language::English => match code {
            "language" => "Language",
            "simulation" => "Simulation",
            "render" => "Render",
            "new" => "New",
            "size" => "Size",
            _ => "UNKNOWN",
        },
        Language::Nederlands => match code {
            "language" => "Taal",
            "simulation" => "Simulatie",
            "render" => "Weergave",
            "new" => "Nieuw",
            "size" => "Afmeting",
            _ => "ONBEKEND",
        },
    }
}
