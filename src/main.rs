#![windows_subsystem = "windows"]

mod cell;
mod element;
mod fill_browser;
mod gui;
mod input;
mod render;
mod sandbox;
mod simulation;
mod toolbox;

use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy_egui::EguiPlugin;
use fill_browser::*;
use gui::gui_system;
use input::{mouse_editor_input, MouseInputState};
use render::render_system;
use sandbox::*;
use simulation::{simulation_system, Simulation};
use toolbox::ToolBox;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Falling Rust".to_string(),
                        width: 1024.,
                        height: 600.,
                        present_mode: bevy::window::PresentMode::Fifo,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(EguiPlugin)
        .add_plugin(FillBrowserWindowPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .init_resource::<MouseInputState>()
        .init_resource::<Simulation>()
        .init_resource::<ToolBox>()
        .add_startup_system(setup)
        .add_system(gui_system)
        .add_system(simulation_system)
        .add_system(render_system)
        .add_system(mouse_editor_input)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // Create an empty texture to fill with our pixels
    commands.spawn(Camera2dBundle::default());

    new_sandbox(commands, images.as_mut(), 512, 512);
}

fn new_sandbox(mut commands: Commands, images: &mut Assets<Image>, width: u32, height: u32) {
    let image = Image::new_fill(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[255, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
    );
    let image_handle = images.add(image);
    commands
        .spawn(SandBox::new(width as usize, height as usize))
        .insert(SpriteBundle {
            texture: image_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });
}
