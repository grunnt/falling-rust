use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseWheel},
        ButtonState,
    },
    prelude::*,
    render::camera::Camera,
};
use bevy_egui::EguiContext;

use crate::{
    element::Element,
    gui::{GuiMode, SandboxGui},
    sandbox::SandBox,
    toolbox::ToolBox,
};

/// Handles both mouse and touch input for the sandbox editor
pub struct PointerInputPlugin;

impl Plugin for PointerInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseInputState>()
            .add_system(mouse_editor_input);
    }
}

#[derive(Default, Resource)]
pub struct MouseInputState {
    pub left_button_down: bool,
    pub middle_button_down: bool,
    pub right_button_down: bool,
    pub position: Vec2,
    pub drag_movement: Vec2,
    pub world_position: Vec2,
}

pub fn mouse_editor_input(
    mut mouse: ResMut<MouseInputState>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera: Query<(&Camera, &mut Transform, &GlobalTransform)>,
    mut egui_context: ResMut<EguiContext>,
    mut toolbox: ResMut<ToolBox>,
    mut sandbox: Query<&mut SandBox>,
    gui: Res<SandboxGui>,
) {
    // Determine button state
    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Left {
            mouse.left_button_down = event.state == ButtonState::Pressed;
        }
        if event.button == MouseButton::Middle {
            mouse.middle_button_down = event.state == ButtonState::Pressed;
        }
        if event.button == MouseButton::Right {
            mouse.right_button_down = event.state == ButtonState::Pressed;
        }
    }

    // Record latest position
    let last_position = mouse.position;
    for event in cursor_moved_events.iter() {
        mouse.position = event.position;
    }
    mouse.drag_movement = if mouse.left_button_down || mouse.middle_button_down {
        last_position - mouse.position
    } else {
        Vec2::ZERO
    };

    // Check mouse wheel
    let mut wheel_y = 0.0;
    for event in mouse_wheel_events.iter() {
        wheel_y += event.y;
    }

    let ctx = egui_context.ctx_mut();
    if ctx.wants_pointer_input() {
        // GUI gets priority input
        mouse.left_button_down = false;
        mouse.middle_button_down = false;
        mouse.right_button_down = false;
        return;
    }

    let sandbox = sandbox.get_single_mut();
    if sandbox.is_err() {
        // Sandbox not active
        return;
    }

    let mut sandbox = sandbox.unwrap();
    // Update world position of the pointer (e.g. for use while editing the world)
    let (camera, mut transform, global_transform) = camera.single_mut();
    let world_pos = camera
        .viewport_to_world(global_transform, mouse.position)
        .unwrap()
        .origin;
    mouse.world_position = Vec2::new(
        world_pos.x + (sandbox.width() / 2) as f32,
        (sandbox.height() / 2) as f32 - world_pos.y,
    );

    // Zoom camera using mouse wheel
    if wheel_y > 0.0 {
        transform.scale.x = (transform.scale.x * 0.9).clamp(0.1, 1.0);
        transform.scale.y = (transform.scale.y * 0.9).clamp(0.1, 1.0);
    } else if wheel_y < 0.0 {
        transform.scale.x = (transform.scale.x * 1.1).clamp(0.1, 1.0);
        transform.scale.y = (transform.scale.y * 1.1).clamp(0.1, 1.0);
    }

    // Pan camera
    let half_width = (sandbox.width() / 2) as f32;
    let half_height = (sandbox.height() / 2) as f32;
    if mouse.middle_button_down || (gui.mode == GuiMode::MoveView && mouse.left_button_down) {
        transform.translation.x += mouse.drag_movement.x * transform.scale.x;
        transform.translation.y += mouse.drag_movement.y * transform.scale.y;

        transform.translation.x = transform.translation.x.clamp(-half_width, half_width);
        transform.translation.y = transform.translation.y.clamp(-half_height, half_height);
    }

    // Edit the world
    if gui.mode != GuiMode::MoveView {
        let (x, y) = (mouse.world_position.x, mouse.world_position.y);
        if x > 0.0 && x < sandbox.width() as f32 && y > 0.0 && y < sandbox.height() as f32 {
            if mouse.left_button_down {
                toolbox.apply(&mut sandbox, x.floor() as usize, y.floor() as usize);
            } else if mouse.right_button_down {
                let element = toolbox.element;
                toolbox.element = Element::Air;
                toolbox.apply(&mut sandbox, x.floor() as usize, y.floor() as usize);
                toolbox.element = element;
            }
        }
    }
}
