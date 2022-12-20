mod cell;
pub mod element;
mod fill_browser;
mod gui;
mod mouse_input;
mod render;
pub mod sandbox;
pub mod simulation;
mod toolbox;
mod touch_input;

use bevy::prelude::*;
use fill_browser::*;
use gui::GuiPlugin;
use mouse_input::MouseInputPlugin;
use render::render_system;
use sandbox::*;
use simulation::{simulation_system, Simulation};
use toolbox::ToolBox;
use touch_input::TouchInputPlugin;

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
        .add_plugin(MouseInputPlugin)
        .add_plugin(TouchInputPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .init_resource::<Simulation>()
        .init_resource::<ToolBox>()
        .add_startup_system(setup)
        .add_system(simulation_system)
        .add_system(render_system)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());
    spawn_sandbox(commands, images.as_mut(), 256, 256);
}
