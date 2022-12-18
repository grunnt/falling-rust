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
use bevy_egui::{
    egui::{self, style::*, FontData, FontDefinitions, FontFamily},
    EguiContext, EguiPlugin,
};
use fill_browser::*;
use gui::gui_system;
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

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut egui_context: ResMut<EguiContext>,
) {
    let mut style = egui::Style::default();
    style.spacing = Spacing::default();
    style.spacing.scroll_bar_width = 20.0;
    style.spacing.button_padding = bevy_egui::egui::Vec2::new(10.0, 10.0);
    egui_context.ctx_mut().set_style(style);

    let mut fonts = FontDefinitions::default();
    let pixelfont_name = "pixelfont";
    let mut pixelfont_data = FontData::from_static(include_bytes!("../pixelfont.ttf"));
    pixelfont_data.tweak.scale = 2.0;
    fonts
        .font_data
        .insert(pixelfont_name.to_owned(), pixelfont_data);
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, pixelfont_name.to_owned());
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .push(pixelfont_name.to_owned());
    egui_context.ctx_mut().set_fonts(fonts);

    commands.spawn(Camera2dBundle::default());
    spawn_sandbox(commands, images.as_mut(), 256, 256);
}
