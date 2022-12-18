mod cell;
pub mod element;
mod fill_browser;
mod gui;
mod input;
mod render;
pub mod sandbox;
pub mod simulation;
mod toolbox;

use bevy::prelude::*;
use fill_browser::*;
use gui::GuiPlugin;
use input::{mouse_editor_input, MouseInputState};
use render::render_system;
use sandbox::*;
use simulation::{simulation_system, Simulation};
use toolbox::ToolBox;

pub fn start_app() {
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
        .add_plugin(FillBrowserWindowPlugin)
        .add_plugin(GuiPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .init_resource::<MouseInputState>()
        .init_resource::<Simulation>()
        .init_resource::<ToolBox>()
        .add_startup_system(setup)
        .add_system(simulation_system)
        .add_system(render_system)
        .add_system(mouse_editor_input)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());
    spawn_sandbox(commands, images.as_mut(), 256, 256);
}
