use bevy::prelude::Resource;

use crate::language::Language;

#[derive(Resource)]
pub struct Settings {
    pub language: Language,
    pub sandbox_size: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            language: Language::English,
            sandbox_size: 256,
        }
    }
}
