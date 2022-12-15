use bevy::{prelude::*, render::render_resource::Extent3d};
use bevy_egui::{egui, EguiContext};

use crate::{
    element::Element,
    new_sandbox,
    sandbox::SandBox,
    simulation::Simulation,
    toolbox::{Tool, ToolBox},
};

pub fn gui_system(
    mut egui_context: ResMut<EguiContext>,
    mut toolbox: ResMut<ToolBox>,
    mut simulation: ResMut<Simulation>,
    mut sandbox: Query<(Entity, &mut SandBox, &Handle<Image>)>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let mut sandbox = sandbox.get_single_mut();
    let sandbox_active = sandbox.is_ok();

    if sandbox_active {
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
    }

    egui::SidePanel::right("right_tools").show(egui_context.ctx_mut(), |ui| {
        if sandbox_active {
            ui.label("Simulation:");
            ui.checkbox(&mut simulation.running, "Running");
            if ui.button("Step").clicked() {
                simulation.step = true;
            }
            ui.label(format!("Step time: {} ms", simulation.frame_time_ms));
            ui.separator();
        }
        ui.label("Sandbox:");
        if sandbox_active {
            if ui.button("Clear").clicked() {
                let (_entity, sandbox, _image_handle) = sandbox.as_mut().unwrap();
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
            let mut image_handle_opt = None;
            if sandbox_active {
                let (entity, _sandbox, existing_image_handle) = sandbox.unwrap();
                // images
                //     .get_mut(existing_image_handle)
                //     .unwrap()
                //     .resize(Extent3d {
                //         width,
                //         height,
                //         depth_or_array_layers: 0,
                //     });
                // image_handle_opt = Some(existing_image_handle.clone());
                commands.entity(entity).despawn();
            }

            new_sandbox(commands, images.as_mut(), image_handle_opt, width, height);
        }
    });
}
