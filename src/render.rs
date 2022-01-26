use crate::{element::Element, sandbox::SandBox};
use bevy::prelude::*;

pub fn level_texture_updater(mut images: ResMut<Assets<Image>>, level: Res<SandBox>) {
    let image = images.get_mut(&level.image_handle).unwrap();
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
            } else if cell.element.is_source() {
                (
                    color.0.min(127) * 2,
                    color.1.min(127) * 2,
                    color.2.min(127) * 2,
                    255,
                )
            } else {
                (color.0, color.1, color.2, 255)
            };
            let bytes_per_pixel = 4;
            let index = (x + y * level.width()) * bytes_per_pixel;
            image.data[index] = color.0;
            image.data[index + 1] = color.1;
            image.data[index + 2] = color.2;
            image.data[index + 3] = color.3;
        }
    }
}
