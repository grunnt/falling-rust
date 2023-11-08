use bevy::prelude::*;
use bevy::utils::Instant;

use crate::sandbox::*;

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
            let color = element_type(sandbox.get_mut(x, y).element).color;
            let index = (x + y * sandbox.width()) * 4;
            image.data[index] = color.0;
            image.data[index + 1] = color.1;
            image.data[index + 2] = color.2;
            image.data[index + 3] = 255;
        }
    }

    let duration = Instant::now() - start;
    sandbox.render_time_ms = duration.as_millis();
}