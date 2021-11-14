mod camera;
mod cell;
mod element;
mod mouse_input;
mod render;
mod sandbox;
mod simulation;
mod toolbox;

use bevy::{
    prelude::*,
    render::texture::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use camera::camera_controller;
use element::Element;
use mouse_input::{mouse_input_handler, MouseInputState};
use render::level_texture_updater;
use sandbox::*;
use simulation::{level_updater, Simulation};
use toolbox::{Tool, ToolBox};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .init_resource::<MouseInputState>()
        .init_resource::<SandBox>()
        .init_resource::<Simulation>()
        .init_resource::<ToolBox>()
        .add_startup_system(setup.system())
        .add_system(gui_system.system().label("gui"))
        .add_system(level_updater.system())
        .add_system(level_texture_updater.system())
        .add_system(mouse_input_handler.system())
        .add_system(camera_controller.system())
        .add_system(level_editor.system().after("gui"))
        .run();
}

fn setup(
    mut commands: Commands,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    level: Res<SandBox>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Create texture for displaying
    let texture = Texture::new_fill(
        Extent3d::new(level.width() as u32, level.height() as u32, 1),
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
    );
    let th = textures.add(texture);

    // Now spawn the sprite for the level -> TODO use handle to update texture?
    commands.spawn().insert_bundle(SpriteBundle {
        material: materials.add(th.into()),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn gui_system(
    egui_context: ResMut<EguiContext>,
    mut toolbox: ResMut<ToolBox>,
    mut simulation: ResMut<Simulation>,
    mut level: ResMut<SandBox>,
) {
    egui::Window::new("Tools").show(egui_context.ctx(), |ui| {
        ui.label("Element:");
        ui.radio_value(&mut toolbox.element, Element::Air, "Air");
        ui.radio_value(&mut toolbox.element, Element::Sand, "Sand");
        ui.radio_value(&mut toolbox.element, Element::Wood, "Wood");
        ui.radio_value(&mut toolbox.element, Element::Rock, "Rock");
        ui.radio_value(&mut toolbox.element, Element::Water, "Water");
        ui.radio_value(&mut toolbox.element, Element::Acid, "Acid");
        ui.radio_value(&mut toolbox.element, Element::Oil, "Oil");
        ui.radio_value(&mut toolbox.element, Element::Lava, "Lava");
        ui.radio_value(&mut toolbox.element, Element::Drain, "Liquid drain");
        ui.radio_value(&mut toolbox.element, Element::Fire, "Fire");
        ui.checkbox(&mut toolbox.source, "Source");
        ui.label("Tool size:");
        ui.add(egui::Slider::new(&mut toolbox.tool_size, 1..=64));
        ui.label("Tool type:");
        ui.radio_value(&mut toolbox.tool, Tool::FillCircle, "Fill Circle");
        ui.radio_value(&mut toolbox.tool, Tool::FillSquare, "Fill Square");
        ui.radio_value(&mut toolbox.tool, Tool::SprayCircle, "Spray Circle");
    });

    egui::Window::new("Simulation").show(egui_context.ctx(), |ui| {
        ui.checkbox(&mut simulation.running, "Running");
        if ui.button("Step").clicked() {
            simulation.step = true;
        }
        ui.label(format!("Frametime: {} ms", simulation.frame_time_ms));
    });

    egui::Window::new("Level").show(egui_context.ctx(), |ui| {
        if ui.button("Clear").clicked() {
            level.clear();
        }
    });
}

fn level_editor(
    mouse: Res<MouseInputState>,
    egui_context: Res<EguiContext>,
    mut toolbox: ResMut<ToolBox>,
    mut level: ResMut<SandBox>,
    mut query: Query<&Transform>,
) {
    if egui_context.ctx().wants_pointer_input() || egui_context.ctx().wants_keyboard_input() {
        // GUI gets priority for input events
        return;
    }
    for transform in query.iter_mut() {
        let x = mouse.world_position.x - transform.translation.x + level.width() as f32 / 2.0;
        let y = level.height() as f32
            - (mouse.world_position.y - transform.translation.y + level.height() as f32 / 2.0);
        if x > 0.0 && x < level.width() as f32 && y > 0.0 && y < level.height() as f32 {
            if mouse.left_button_down {
                toolbox.apply(&mut level, x.floor() as usize, y.floor() as usize);
            }
            if mouse.right_button_down {
                let element = toolbox.element;
                toolbox.element = Element::Air;
                toolbox.apply(&mut level, x.floor() as usize, y.floor() as usize);
                toolbox.element = element;
            }
        }
    }
}
