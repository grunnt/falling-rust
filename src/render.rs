use bevy::prelude::*;
use bevy::utils::Instant;

use crate::cell::Cell;
use crate::element::{element_type, RenderMethod};
use crate::pseudo_random::PseudoRandom;
use crate::sandbox::SandBox;

#[derive(Resource)]
pub struct RenderState {
    pub random: PseudoRandom,
}

// "Render" the world by copying the element cells to pixels on a texture
pub fn render_system(
    mut images: ResMut<Assets<Image>>,
    mut render_state: ResMut<RenderState>,
    mut sandbox: Query<(&mut SandBox, &Handle<Image>)>,
) {
    let sandbox = sandbox.get_single_mut();
    if sandbox.is_err() {
        // Sandbox not active, so skip this
        return;
    }
    let (mut sandbox, image_handle) = sandbox.unwrap();

    let random = &mut render_state.as_mut().random;

    let start = Instant::now();

    let image = images.get_mut(image_handle).unwrap();
    for y in 0..sandbox.height() {
        for x in 0..sandbox.width() {
            let cell = sandbox.get_mut(x, y);
            let color = cell_color(cell, random);
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

pub fn cell_color(cell: &mut Cell, random: &mut PseudoRandom) -> (u8, u8, u8) {
    let element_type = element_type(cell.element);
    let color = match element_type.render {
        RenderMethod::FixedColor => element_type.color_1,
        RenderMethod::StrengthLinear => interpolate(
            &element_type.color_1,
            &element_type.color_2,
            cell.strength,
            element_type.strength,
        ),
        RenderMethod::VariantLinear => interpolate(
            &element_type.color_1,
            &element_type.color_2,
            cell.variant,
            u8::MAX,
        ),
        RenderMethod::Flicker => {
            cell.variant = ((cell.variant as u32 + random.next()) % 255) as u8;
            interpolate(
                &element_type.color_1,
                &element_type.color_2,
                cell.variant,
                u8::MAX,
            )
        }
    };
    color
}

pub fn interpolate(
    color_1: &(u8, u8, u8),
    color_2: &(u8, u8, u8),
    factor: u8,
    max: u8,
) -> (u8, u8, u8) {
    let factor_f32 = factor as f32 / max as f32;
    let inv_factor_f32 = 1.0 - factor_f32;
    (
        (color_1.0 as f32 * factor_f32 + color_2.0 as f32 * inv_factor_f32) as u8,
        (color_1.1 as f32 * factor_f32 + color_2.1 as f32 * inv_factor_f32) as u8,
        (color_1.2 as f32 * factor_f32 + color_2.2 as f32 * inv_factor_f32) as u8,
    )
}
