use bevy::prelude::*;
use bevy_egui::{
    egui::{self, style::*, FontData, FontDefinitions, FontFamily, ScrollArea},
    EguiContext, EguiPlugin,
};

use crate::{
    element::Element,
    sandbox::SandBox,
    simulation::Simulation,
    spawn_sandbox,
    toolbox::{Tool, ToolBox},
};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system(gui_system)
            .add_startup_system(setup_gui);
    }
}

pub fn gui_system(
    mut egui_context: ResMut<EguiContext>,
    mut toolbox: ResMut<ToolBox>,
    mut simulation: ResMut<Simulation>,
    mut sandbox: Query<(Entity, &mut SandBox)>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let mut sandbox = sandbox.get_single_mut();
    let sandbox_active = sandbox.is_ok();

    if sandbox_active {
        egui::SidePanel::left("left_tools")
            .resizable(false)
            .default_width(220.0)
            .show(egui_context.ctx_mut(), |ui| {
                ScrollArea::vertical()
                    .max_height(f32::INFINITY)
                    .show(ui, |ui| {
                        ui.with_layout(
                            egui::Layout::top_down_justified(egui::Align::Center),
                            |ui| {
                                ui.label("Element:");
                                ui.selectable_value(&mut toolbox.element, Element::Air, "Air");
                                ui.selectable_value(&mut toolbox.element, Element::Sand, "Sand");
                                ui.selectable_value(&mut toolbox.element, Element::Wood, "Wood");
                                ui.selectable_value(&mut toolbox.element, Element::Iron, "Iron");
                                ui.selectable_value(&mut toolbox.element, Element::Rock, "Rock");
                                ui.selectable_value(&mut toolbox.element, Element::Water, "Water");
                                ui.selectable_value(&mut toolbox.element, Element::Acid, "Acid");
                                ui.selectable_value(&mut toolbox.element, Element::Oil, "Oil");
                                ui.selectable_value(&mut toolbox.element, Element::Lava, "Lava");
                                ui.selectable_value(&mut toolbox.element, Element::Fire, "Fire");
                                ui.selectable_value(&mut toolbox.element, Element::Life, "Life");
                                ui.selectable_value(&mut toolbox.element, Element::Seed, "Seed");
                                ui.selectable_value(&mut toolbox.element, Element::TNT, "TNT");
                                ui.selectable_value(&mut toolbox.element, Element::Fuse, "Fuse");
                                ui.selectable_value(
                                    &mut toolbox.element,
                                    Element::WaterSource,
                                    "Water source",
                                );
                                ui.selectable_value(
                                    &mut toolbox.element,
                                    Element::AcidSource,
                                    "Acid Source",
                                );
                                ui.selectable_value(
                                    &mut toolbox.element,
                                    Element::OilSource,
                                    "Oil source",
                                );
                                ui.selectable_value(
                                    &mut toolbox.element,
                                    Element::LavaSource,
                                    "Lava source",
                                );
                                ui.selectable_value(
                                    &mut toolbox.element,
                                    Element::FireSource,
                                    "Fire source",
                                );
                                ui.selectable_value(
                                    &mut toolbox.element,
                                    Element::Drain,
                                    "Liquid drain",
                                );
                                ui.separator();
                                ui.label("Tool:");
                                ui.selectable_value(&mut toolbox.tool, Tool::Pixel, "Pixel");
                                ui.selectable_value(&mut toolbox.tool, Tool::FillCircle, "Circle");
                                ui.selectable_value(&mut toolbox.tool, Tool::FillSquare, "Square");
                                ui.selectable_value(&mut toolbox.tool, Tool::SprayCircle, "Spray");
                                ui.separator();
                                ui.label("Size:");
                                ui.add(egui::Slider::new(&mut toolbox.tool_size, 1..=64));
                            },
                        );
                    });
            });
    }

    egui::SidePanel::right("right_tools")
        .resizable(false)
        .default_width(250.0)
        .show(egui_context.ctx_mut(), |ui| {
            ScrollArea::vertical()
                .max_height(f32::INFINITY)
                .show(ui, |ui| {
                    ui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            if sandbox_active {
                                let (_entity, sandbox) = sandbox.as_mut().unwrap();
                                ui.label("Simulation:");
                                ui.checkbox(&mut simulation.running, "Running");
                                if !simulation.running {
                                    if ui.button("Step").clicked() {
                                        simulation.step = true;
                                    }
                                }
                                ui.label(format!("Simulation: {} ms", simulation.frame_time_ms));
                                ui.label(format!("Render: {} ms", sandbox.render_time_ms));
                                ui.separator();
                            }
                            ui.label("Sandbox:");
                            if sandbox_active {
                                if ui.button("Clear").clicked() {
                                    let (_entity, sandbox) = sandbox.as_mut().unwrap();
                                    sandbox.clear();
                                }
                            }
                            let mut new_size = None;
                            if ui.button("New 64x64").clicked() {
                                new_size = Some((64, 64));
                            }
                            if ui.button("New 128x128").clicked() {
                                new_size = Some((128, 128));
                            }
                            if ui.button("New 256x256").clicked() {
                                new_size = Some((256, 256));
                            }
                            if ui.button("New 512x512").clicked() {
                                new_size = Some((512, 512));
                            }
                            if ui.button("New 1024x1024").clicked() {
                                new_size = Some((1024, 1024));
                            }
                            if let Some((width, height)) = new_size {
                                if sandbox_active {
                                    let (entity, _sandbox) = sandbox.unwrap();
                                    commands.entity(entity).despawn();
                                }

                                spawn_sandbox(commands, images.as_mut(), width, height);
                            }
                        },
                    );
                });
        });
}

fn setup_gui(mut egui_context: ResMut<EguiContext>) {
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
}
