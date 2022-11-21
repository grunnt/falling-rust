use bevy::{
    prelude::*,
    render::camera::{Camera, OrthographicProjection},
};

use crate::mouse_input::MouseInputState;

pub fn camera_controller(
    state: Res<MouseInputState>,
    mut cameras: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
) {
    // Zoom using mouse wheel
    if state.wheel_delta != 0.0 {
        if let Some((mut camera, _)) = cameras.iter_mut().next() {
            camera.scale.x = (camera.scale.x - state.wheel_delta * 0.1).clamp(0.1, 1.0);
            camera.scale.y = (camera.scale.y - state.wheel_delta * 0.1).clamp(0.1, 1.0);
        }
    }
    // Drag using middle mouse button
    if state.middle_button_down && state.position_delta != Vec2::ZERO {
        if let Some((mut camera, _)) = cameras.iter_mut().next() {
            camera.translation.x = camera.translation.x - state.position_delta.x;
            camera.translation.y = camera.translation.y + state.position_delta.y;
        }
    }
}
