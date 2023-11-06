use bevy::{prelude::*, window::WindowResolution};
use fill_browser::*;
use gui::GuiPlugin;
use pointer_input::PointerInputPlugin;
use pseudo_random::PseudoRandom;
use render::{render_system, RenderState};
use sandbox::*;
use simulation::{Simulation, simulation_system};
use toolbox::ToolBox;

mod cell;
pub mod element;
mod fill_browser;
mod gui;
mod pointer_input;
mod pseudo_random;
mod render;
pub mod sandbox;
pub mod simulation;
mod toolbox;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum SystemOrderLabel {
    PointerInput,
}

pub fn start_app() {
    App::new()
        .add_plugins(
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
        )
        .add_plugins(FillBrowserWindowPlugin)
        .add_plugins(GuiPlugin)
        .add_plugins(PointerInputPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .init_resource::<Simulation>()
        .init_resource::<ToolBox>()
        .insert_resource(RenderState {
            random: PseudoRandom::new(),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (simulation_system, render_system).chain())
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());
    spawn_sandbox(commands, images.as_mut(), 256, 256);
}
