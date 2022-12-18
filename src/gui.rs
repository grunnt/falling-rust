use bevy::prelude::*;
use bevy_egui::{
    egui::{self, style::*, FontData, FontDefinitions, FontFamily, ScrollArea},
    EguiContext, EguiPlugin,
};

use crate::{
    element::*,
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
            .insert_resource(SandboxGui {
                mode: GuiMode::Closed,
            })
            .add_startup_system(setup_gui);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GuiMode {
    Closed,
    ElementSelect,
    ToolSelect,
    SandboxSettings,
    View,
}

#[derive(Resource)]
pub struct SandboxGui {
    pub mode: GuiMode,
}

pub fn gui_system(
    mut egui_context: ResMut<EguiContext>,
    mut gui: ResMut<SandboxGui>,
    mut toolbox: ResMut<ToolBox>,
    mut simulation: ResMut<Simulation>,
    mut sandbox: Query<(Entity, &mut SandBox)>,
    commands: Commands,
    images: ResMut<Assets<Image>>,
) {
    let (entity, mut sandbox) = sandbox.single_mut();

    edit_gui(&mut egui_context, toolbox.as_ref(), &mut gui);
    match gui.mode {
        GuiMode::ElementSelect => select_element_gui(&mut egui_context, toolbox.as_mut()),
        GuiMode::ToolSelect => select_tool_gui(&mut egui_context, toolbox.as_mut()),
        GuiMode::SandboxSettings => sandbox_settings_gui(
            egui_context.as_mut(),
            simulation.as_mut(),
            sandbox.as_mut(),
            commands,
            entity,
            images,
            &mut gui,
        ),
        GuiMode::View => {}
        GuiMode::Closed => {}
    }
}

fn edit_gui(egui_context: &mut EguiContext, toolbox: &ToolBox, gui: &mut SandboxGui) {
    egui::TopBottomPanel::bottom("edit_gui")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                let previous_mode = gui.mode;
                select_mode(
                    GuiMode::ElementSelect,
                    format!("{}", toolbox.element.to_string()).as_str(),
                    ui,
                    gui,
                    previous_mode,
                );
                let tool_title = if toolbox.tool == Tool::Pixel {
                    format!("{}", toolbox.tool.to_string())
                } else {
                    format!("{} ({})", toolbox.tool.to_string(), toolbox.tool_size)
                };
                select_mode(
                    GuiMode::ToolSelect,
                    tool_title.as_str(),
                    ui,
                    gui,
                    previous_mode,
                );
                select_mode(GuiMode::SandboxSettings, "Sandbox", ui, gui, previous_mode);
                select_mode(GuiMode::View, "View", ui, gui, previous_mode);
            });
        });
}

fn select_mode(
    mode: GuiMode,
    name: &str,
    ui: &mut egui::Ui,
    gui: &mut SandboxGui,
    previous_mode: GuiMode,
) {
    if ui.selectable_value(&mut gui.mode, mode, name).clicked() {
        if previous_mode == mode {
            gui.mode = GuiMode::Closed;
        }
    }
}

fn sandbox_settings_gui(
    egui_context: &mut EguiContext,
    simulation: &mut Simulation,
    sandbox: &mut SandBox,
    mut commands: Commands,
    entity: Entity,
    mut images: ResMut<Assets<Image>>,
    gui: &mut SandboxGui,
) {
    egui::SidePanel::right("sandbox_settings_gui")
        .resizable(false)
        .default_width(250.0)
        .show(egui_context.ctx_mut(), |ui| {
            ScrollArea::vertical()
                .max_height(f32::INFINITY)
                .show(ui, |ui| {
                    ui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
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
                            ui.label("Sandbox:");
                            if ui.button("Clear").clicked() {
                                sandbox.clear();
                                gui.mode = GuiMode::Closed;
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
                                commands.entity(entity).despawn();
                                spawn_sandbox(commands, images.as_mut(), width, height);
                                gui.mode = GuiMode::Closed;
                            }
                        },
                    );
                });
        });
}

fn select_element_gui(egui_context: &mut EguiContext, toolbox: &mut ToolBox) {
    egui::SidePanel::left("select_element_gui")
        .resizable(false)
        .default_width(250.0)
        .show(egui_context.ctx_mut(), |ui| {
            ScrollArea::vertical()
                .max_height(f32::INFINITY)
                .show(ui, |ui| {
                    ui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            ui.label("Element:");
                            ui.separator();
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
                        },
                    );
                });
        });
}

fn select_tool_gui(egui_context: &mut EguiContext, toolbox: &mut ToolBox) {
    egui::SidePanel::left("select_tool_gui")
        .resizable(false)
        .default_width(220.0)
        .show(egui_context.ctx_mut(), |ui| {
            ScrollArea::vertical()
                .max_height(f32::INFINITY)
                .show(ui, |ui| {
                    ui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            ui.label("Tool:");
                            ui.separator();
                            ui.selectable_value(&mut toolbox.tool, Tool::Pixel, "Pixel");
                            ui.selectable_value(&mut toolbox.tool, Tool::Circle, "Circle");
                            ui.selectable_value(&mut toolbox.tool, Tool::Square, "Square");
                            ui.selectable_value(&mut toolbox.tool, Tool::Spray, "Spray");
                            ui.separator();
                            if toolbox.tool != Tool::Pixel {
                                ui.label("Size:");
                                ui.separator();
                                ui.add(egui::Slider::new(&mut toolbox.tool_size, 1..=64));
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
