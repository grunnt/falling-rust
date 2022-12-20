use bevy::{prelude::*, render::camera::Camera};
use bevy_egui::EguiContext;

use crate::gui::{GuiMode, SandboxGui};

pub struct TouchInputPlugin;

impl Plugin for TouchInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TouchInputState>()
            .add_system(touch_editor_input);
    }
}

#[derive(Default, Resource)]
pub struct TouchInputState {
    pub left_button_down: bool,
    pub middle_button_down: bool,
    pub right_button_down: bool,
    pub position: Vec2,
    pub world_position: Vec2,
}

pub fn touch_editor_input(
    touches: Res<Touches>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut egui_context: ResMut<EguiContext>,
    gui: Res<SandboxGui>,
) {
    let ctx = egui_context.ctx_mut();
    if ctx.is_using_pointer()
        || ctx.is_pointer_over_area()
        || ctx.wants_pointer_input()
        || ctx.wants_keyboard_input()
    {
        // GUI gets priority for input events
        return;
    }

    // Update camera
    if gui.mode == GuiMode::View {
        let mut transform = camera.single_mut();
        let touch_count = touches.iter().count();
        match touch_count {
            1 => {
                // Pan
                let touch = touches.iter().next().unwrap();
                transform.translation.x =
                    transform.translation.x - touch.delta().x * transform.scale.x;
                transform.translation.y =
                    transform.translation.y + touch.delta().y * transform.scale.y;
            }
            2 => {
                // Pinch zoom
            }
            _ => {}
        }
    }
}
