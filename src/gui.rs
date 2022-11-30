use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{
    element::Element,
    sandbox::SandBox,
    simulation::Simulation,
    toolbox::{Tool, ToolBox},
};

pub fn gui_system(
    mut egui_context: ResMut<EguiContext>,
    mut toolbox: ResMut<ToolBox>,
    mut simulation: ResMut<Simulation>,
    mut level: ResMut<SandBox>,
) {
    egui::SidePanel::left("left_tools")
        .min_width(180.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.label("Tool element:");
            ui.radio_value(&mut toolbox.element, Element::Air, "Air");
            ui.radio_value(&mut toolbox.element, Element::Sand, "Sand");
            ui.radio_value(&mut toolbox.element, Element::Wood, "Wood");
            ui.radio_value(&mut toolbox.element, Element::Iron, "Iron");
            ui.radio_value(&mut toolbox.element, Element::Rock, "Rock");
            ui.radio_value(&mut toolbox.element, Element::Water, "Water");
            ui.radio_value(&mut toolbox.element, Element::Acid, "Acid");
            ui.radio_value(&mut toolbox.element, Element::Oil, "Oil");
            ui.radio_value(&mut toolbox.element, Element::Lava, "Lava");
            ui.radio_value(&mut toolbox.element, Element::Fire, "Fire");
            ui.radio_value(&mut toolbox.element, Element::Life, "Life");
            ui.radio_value(&mut toolbox.element, Element::Seed, "Seed");
            ui.radio_value(&mut toolbox.element, Element::WaterSource, "Water source");
            ui.radio_value(&mut toolbox.element, Element::AcidSource, "Acid Source");
            ui.radio_value(&mut toolbox.element, Element::OilSource, "Oil source");
            ui.radio_value(&mut toolbox.element, Element::LavaSource, "Lava source");
            ui.radio_value(&mut toolbox.element, Element::FireSource, "Fire source");
            ui.radio_value(&mut toolbox.element, Element::Drain, "Liquid drain");
            ui.separator();
            ui.label("Tool size:");
            ui.add(egui::Slider::new(&mut toolbox.tool_size, 1..=64));
            ui.separator();
            ui.label("Tool shape:");
            ui.radio_value(&mut toolbox.tool, Tool::FillCircle, "Fill Circle");
            ui.radio_value(&mut toolbox.tool, Tool::FillSquare, "Fill Square");
            ui.radio_value(&mut toolbox.tool, Tool::SprayCircle, "Spray Circle");
        });

    egui::SidePanel::right("right_tools").show(egui_context.ctx_mut(), |ui| {
        ui.label("Simulation:");
        ui.checkbox(&mut simulation.running, "Running");
        if ui.button("Step").clicked() {
            simulation.step = true;
        }
        ui.label(format!("Step time: {} ms", simulation.frame_time_ms));
        ui.separator();
        ui.label("Sandbox:");
        if ui.button("Clear").clicked() {
            level.clear();
        }
    });
}
