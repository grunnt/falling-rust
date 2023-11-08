// Hide console on Windows. Remove to get console logging and backtraces.
#![windows_subsystem = "windows"]

use bevy::{prelude::*, window::WindowResolution};

use render::render_system;
use sandbox::*;

use crate::interface::InterfacePlugin;
use crate::simulation::{Simulation, simulation_system};

mod pseudo_random;
mod render;
mod sandbox;
mod simulation;
mod interface;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum SystemOrderLabel {
    PointerInput,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Falling Rust".to_string(),
                        resolution: WindowResolution::new(1024.0, 600.0),
                        present_mode: bevy::window::PresentMode::Fifo,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            InterfacePlugin
        ))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .init_resource::<Simulation>()
        .add_systems(Startup, setup)
        .add_systems(Update, (simulation_system, render_system).chain())
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());
    spawn_sandbox(commands, images.as_mut(), 256, 256);
}
