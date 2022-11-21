use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
        ButtonState,
    },
    math::Vec3Swizzles,
    prelude::*,
    render::camera::{Camera, OrthographicProjection},
};

#[derive(Default)]
pub struct MouseInputState {
    pub left_button_down: bool,
    pub middle_button_down: bool,
    pub right_button_down: bool,
    pub left_button_just_pressed: bool,
    pub middle_button_just_pressed: bool,
    pub right_button_just_pressed: bool,
    pub position: Vec2,
    pub world_position: Vec2,
    pub position_delta: Vec2,
    pub world_position_delta: Vec2,
    pub wheel_delta: f32,
}

pub fn mouse_input_handler(
    mut state: ResMut<MouseInputState>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    cameras: Query<(&GlobalTransform, &OrthographicProjection), With<Camera>>,
) {
    // Reset per-frame mouse state
    state.left_button_just_pressed = false;
    state.middle_button_just_pressed = false;
    state.right_button_just_pressed = false;
    state.wheel_delta = 0.0;
    state.position_delta = Vec2::ZERO;
    state.world_position_delta = Vec2::ZERO;

    for event in mouse_button_input_events.iter() {
        let button_down = event.state == ButtonState::Pressed;
        match event.button {
            MouseButton::Left => {
                if button_down && !state.left_button_down {
                    state.left_button_just_pressed = true;
                }
                state.left_button_down = button_down;
            }
            MouseButton::Middle => {
                if button_down && !state.middle_button_down {
                    state.middle_button_just_pressed = true;
                }
                state.middle_button_down = button_down;
            }
            MouseButton::Right => {
                if button_down && !state.right_button_down {
                    state.right_button_just_pressed = true;
                }
                state.right_button_down = button_down;
            }
            _ => {}
        }
    }

    for event in mouse_wheel_events.iter() {
        state.wheel_delta += event.y;
    }

    for event in mouse_motion_events.iter() {
        state.position_delta = event.delta;
    }

    for event in cursor_moved_events.iter() {
        state.position = event.position;
    }

    if let Some((camera, projection)) = cameras.iter().next() {
        state.world_position = camera
            .mul_vec3(
                state.position.extend(0.0)
                    + Vec3::new(projection.left, projection.bottom, projection.near),
            )
            .xy();
        state.world_position_delta = camera
            .mul_vec3(
                state.position_delta.extend(0.0)
                    + Vec3::new(projection.left, projection.bottom, projection.near),
            )
            .xy();
    }
}
