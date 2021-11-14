use crate::{element::Element, sandbox::SandBox};
use bevy::prelude::*;

pub fn level_texture_updater(
    materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    query: Query<&Handle<ColorMaterial>>,
    level: Res<SandBox>,
) {
    for material_handle in query.iter() {
        let texture_handle = materials
            .get(material_handle)
            .unwrap()
            .texture
            .as_ref()
            .unwrap();
        let texture = textures.get_mut(texture_handle).unwrap();
        for y in 0..level.height() {
            for x in 0..level.width() {
                let cell = level.get(x, y);
                let color = cell.element.color();
                let randomize_color_factor = cell.element.randomize_color_factor();
                let color = if cell.element == Element::Smoke {
                    let factor = 1.0 - (cell.strength as f32 / Element::Smoke.strength() as f32);
                    let red = color.0 as f32 * factor;
                    let green = color.1 as f32 * factor;
                    let blue = color.2 as f32 * factor;
                    (red as u8, green as u8, blue as u8, 255)
                } else if randomize_color_factor > 0.0 {
                    let remainder = 1.0 - randomize_color_factor;
                    let factor = remainder + (cell.variant as f32 / 255.0) * randomize_color_factor;
                    let red = color.0 as f32 * factor;
                    let green = color.1 as f32 * factor;
                    let blue = color.2 as f32 * factor;
                    (red as u8, green as u8, blue as u8, 255)
                } else {
                    (color.0, color.1, color.2, 255)
                };
                let bytes_per_pixel = 4;
                let index = (x + y * texture.size.width as usize) * bytes_per_pixel;
                texture.data[index] = color.0;
                texture.data[index + 1] = color.1;
                texture.data[index + 2] = color.2;
                texture.data[index + 3] = color.3;
            }
        }
    }
}
