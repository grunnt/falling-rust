use crate::cell::Cell;
use crate::{element::Element, sandbox::SandBox};
use bevy::prelude::*;
use bevy::utils::Instant;

// "Render" the world by copying the element cells to pixels on a texture
pub fn render_system(
    mut images: ResMut<Assets<Image>>,
    mut sandbox: Query<(&mut SandBox, &Handle<Image>)>,
) {
    let sandbox = sandbox.get_single_mut();
    if sandbox.is_err() {
        // Sandbox not active, so skip this
        return;
    }
    let (mut sandbox, image_handle) = sandbox.unwrap();

    let start = Instant::now();

    let image = images.get_mut(image_handle).unwrap();
    for y in 0..sandbox.height() {
        for x in 0..sandbox.width() {
            let cell = sandbox.get(x, y);
            let color = cell_color(cell);
            let bytes_per_pixel = 4;
            let index = (x + y * sandbox.width()) * bytes_per_pixel;
            image.data[index] = color.0;
            image.data[index + 1] = color.1;
            image.data[index + 2] = color.2;
            image.data[index + 3] = 255;
        }
    }

    let duration = Instant::now() - start;
    sandbox.render_time_ms = duration.as_millis();
}

pub fn cell_color(cell: &Cell) -> (u8, u8, u8) {
    let color = cell.element.color();
    let randomize_color_factor = cell.element.randomize_color_factor();
    let color = if cell.element == Element::Smoke {
        let factor = 1.0 - (cell.strength as f32 / cell.element.strength() as f32);
        let red = color.0 as f32 * factor;
        let green = color.1 as f32 * factor;
        let blue = color.2 as f32 * factor;
        (red as u8, green as u8, blue as u8)
    } else if randomize_color_factor > 0.0 {
        let remainder = 1.0 - randomize_color_factor;
        let factor = remainder + (cell.variant as f32 / 255.0) * randomize_color_factor;
        let red = color.0 as f32 * factor;
        let green = color.1 as f32 * factor;
        let blue = color.2 as f32 * factor;
        (red as u8, green as u8, blue as u8)
    } else if cell.element.is_source() {
        (
            color.0.min(127) * 2,
            color.1.min(127) * 2,
            color.2.min(127) * 2,
        )
    } else {
        (color.0, color.1, color.2)
    };
    color
}
