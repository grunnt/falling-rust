mod cell;
pub mod element;
mod fill_browser;
mod gui;
mod language;
mod pointer_input;
mod pseudo_random;
mod render;
pub mod sandbox;
mod settings;
pub mod simulation;
mod toolbox;

use bevy::prelude::*;
use fill_browser::*;
use gui::GuiPlugin;
use pointer_input::PointerInputPlugin;
use render::render_system;
use sandbox::*;
use settings::Settings;
use simulation::{simulation_system, Simulation};
use toolbox::ToolBox;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum SystemOrderLabel {
    PointerInput,
}

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
        .add_plugin(PointerInputPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .init_resource::<Simulation>()
        .init_resource::<ToolBox>()
        .init_resource::<Settings>()
        .add_startup_system(setup)
        .add_system(simulation_system)
        .add_system(render_system)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());
    spawn_sandbox(commands, images.as_mut(), 256, 256);
}
