use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self, style::*, Align2, Color32, ColorImage, FontData, FontDefinitions, FontFamily,
        ScrollArea, TextureHandle, Ui,
    },
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
            .add_startup_system(setup_gui);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GuiMode {
    MainGui,
    ElementSelect,
    ToolSelect,
    SandboxSettings,
    MoveView,
}

#[derive(Resource)]
pub struct SandboxGui {
    pub mode: GuiMode,
    pub last_element: Element,
    pub bucket_icon_handle: TextureHandle,
    pub icon_circle_handle: TextureHandle,
    pub icon_square_handle: TextureHandle,
    pub icon_pencil_handle: TextureHandle,
    pub icon_spray_handle: TextureHandle,
    pub icon_play_handle: TextureHandle,
    pub icon_pause_handle: TextureHandle,
    pub icon_zoom_in_handle: TextureHandle,
    pub icon_zoom_out_handle: TextureHandle,
    pub icon_move_handle: TextureHandle,
    pub icon_settings_handle: TextureHandle,
    pub icon_empty_handle: TextureHandle,
    pub icon_eraser_handle: TextureHandle,
}

pub fn gui_system(
    mut egui_context: ResMut<EguiContext>,
    camera: Query<&mut Transform, With<Camera>>,
    mut gui: ResMut<SandboxGui>,
    mut toolbox: ResMut<ToolBox>,
    mut simulation: ResMut<Simulation>,
    mut sandbox: Query<(Entity, &mut SandBox)>,
    commands: Commands,
    images: ResMut<Assets<Image>>,
) {
    egui::Area::new("play_pause")
        .anchor(Align2::RIGHT_TOP, [-15.0, 15.0])
        .show(egui_context.ctx_mut(), |ui| {
            if ui
                .add(
                    egui::widgets::ImageButton::new(
                        if simulation.running {
                            &gui.icon_play_handle
                        } else {
                            &gui.icon_pause_handle
                        },
                        [64.0, 64.0],
                    )
                    .frame(false),
                )
                .clicked()
            {
                simulation.running = !simulation.running;
            };
        });

    navigation_gui(egui_context.as_mut(), gui.as_mut(), camera);

    tool_gui(egui_context.as_mut(), gui.as_mut(), toolbox.as_mut());

    if gui.mode == GuiMode::ElementSelect {
        egui::Area::new("element_select_menu")
            .anchor(Align2::LEFT_TOP, [15.0, 15.0])
            .show(egui_context.ctx_mut(), |ui| {
                ui.horizontal_wrapped(|ui| {
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
                    ui.selectable_value(&mut toolbox.element, Element::WaterSource, "Water source");
                    ui.selectable_value(&mut toolbox.element, Element::AcidSource, "Acid Source");
                    ui.selectable_value(&mut toolbox.element, Element::OilSource, "Oil source");
                    ui.selectable_value(&mut toolbox.element, Element::LavaSource, "Lava source");
                    ui.selectable_value(&mut toolbox.element, Element::FireSource, "Fire source");
                    ui.selectable_value(&mut toolbox.element, Element::Drain, "Liquid drain");
                });
            });
    }
    if gui.mode == GuiMode::ToolSelect {
        egui::Area::new("tool_select_menu")
            .anchor(Align2::LEFT_TOP, [15.0, 15.0])
            .show(egui_context.ctx_mut(), |ui| {
                ui.horizontal_wrapped(|ui| {
                    if ui
                        .add(
                            egui::widgets::ImageButton::new(&gui.icon_pencil_handle, [64.0, 64.0])
                                .frame(false),
                        )
                        .clicked()
                    {
                        toolbox.tool = Tool::Pixel;
                        gui.mode = GuiMode::MainGui;
                    };
                    if ui
                        .add(
                            egui::widgets::ImageButton::new(&gui.icon_circle_handle, [64.0, 64.0])
                                .frame(false),
                        )
                        .clicked()
                    {
                        toolbox.tool = Tool::Circle;
                        gui.mode = GuiMode::MainGui;
                    };
                    if ui
                        .add(
                            egui::widgets::ImageButton::new(&gui.icon_square_handle, [64.0, 64.0])
                                .frame(false),
                        )
                        .clicked()
                    {
                        toolbox.tool = Tool::Square;
                        gui.mode = GuiMode::MainGui;
                    };
                    if ui
                        .add(
                            egui::widgets::ImageButton::new(&gui.icon_spray_handle, [64.0, 64.0])
                                .frame(false),
                        )
                        .clicked()
                    {
                        toolbox.tool = Tool::Spray;
                        gui.mode = GuiMode::MainGui;
                    };
                    if toolbox.tool != Tool::Pixel {
                        ui.add(egui::Slider::new(&mut toolbox.tool_size, 1..=64));
                    }
                });
            });
    }
}

fn navigation_gui(
    egui_context: &mut EguiContext,
    gui: &mut SandboxGui,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    egui::Area::new("view")
        .anchor(Align2::RIGHT_BOTTOM, [-15.0, -15.0])
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                if ui
                    .add(
                        egui::widgets::ImageButton::new(&gui.icon_zoom_in_handle, [64.0, 64.0])
                            .frame(false),
                    )
                    .clicked()
                {
                    let mut transform = camera.single_mut();
                    transform.scale.x = (transform.scale.x * 0.9).clamp(0.1, 1.0);
                    transform.scale.y = (transform.scale.y * 0.9).clamp(0.1, 1.0);
                };
                if ui
                    .add(
                        egui::widgets::ImageButton::new(&gui.icon_zoom_out_handle, [64.0, 64.0])
                            .frame(false),
                    )
                    .clicked()
                {
                    let mut transform = camera.single_mut();
                    transform.scale.x = (transform.scale.x * 1.1).clamp(0.1, 1.0);
                    transform.scale.y = (transform.scale.y * 1.1).clamp(0.1, 1.0);
                };
                let move_button =
                    egui::widgets::ImageButton::new(&gui.icon_move_handle, [64.0, 64.0])
                        .frame(false);
                let move_button = if gui.mode == GuiMode::MoveView {
                    move_button.tint(Color32::LIGHT_GREEN)
                } else {
                    move_button
                };
                if ui.add(move_button).clicked() {
                    gui.mode = if gui.mode == GuiMode::MoveView {
                        GuiMode::MainGui
                    } else {
                        GuiMode::MoveView
                    }
                };
            });
        });
}

fn tool_gui(egui_context: &mut EguiContext, gui: &mut SandboxGui, toolbox: &mut ToolBox) {
    egui::Area::new("tool_select")
        .anchor(Align2::LEFT_BOTTOM, [15.0, -15.0])
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                let eraser_button =
                    egui::widgets::ImageButton::new(&gui.icon_eraser_handle, [64.0, 64.0])
                        .frame(false);
                let eraser_button = if toolbox.element == Element::Air {
                    eraser_button.tint(Color32::LIGHT_GREEN)
                } else {
                    eraser_button
                };
                if ui.add(eraser_button).clicked() {
                    if toolbox.element == Element::Air {
                        toolbox.element = gui.last_element;
                    } else {
                        toolbox.element = Element::Air;
                    }
                };

                element_gui(ui, gui, toolbox);

                let tool_button = egui::widgets::ImageButton::new(
                    match toolbox.tool {
                        Tool::Pixel => &gui.icon_pencil_handle,
                        Tool::Circle => &gui.icon_circle_handle,
                        Tool::Square => &gui.icon_square_handle,
                        Tool::Spray => &gui.icon_spray_handle,
                    },
                    [64.0, 64.0],
                )
                .frame(false);
                let tool_button = if gui.mode == GuiMode::ToolSelect {
                    tool_button.tint(Color32::LIGHT_GREEN)
                } else {
                    tool_button
                };
                if ui.add(tool_button).clicked() {
                    if gui.mode == GuiMode::ToolSelect {
                        gui.mode = GuiMode::MainGui;
                    } else {
                        gui.mode = GuiMode::ToolSelect;
                    }
                };
            });
        });
}

fn element_gui(ui: &mut Ui, gui: &mut SandboxGui, toolbox: &mut ToolBox) {
    let element_button =
        egui::widgets::ImageButton::new(&gui.icon_empty_handle, [64.0, 64.0]).frame(false);
    let element_button = if gui.mode == GuiMode::ElementSelect {
        element_button.tint(Color32::LIGHT_GREEN)
    } else {
        element_button
    };
    if ui.add(element_button).clicked() {
        if gui.mode == GuiMode::ElementSelect {
            gui.mode = GuiMode::MainGui;
        } else {
            gui.mode = GuiMode::ElementSelect;
        }
    };
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
                                gui.mode = GuiMode::MainGui;
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
                                gui.mode = GuiMode::MainGui;
                            }
                        },
                    );
                });
        });
}

fn setup_gui(mut commands: Commands, mut egui_context: ResMut<EguiContext>) {
    // General styling
    let mut style = egui::Style::default();
    style.spacing = Spacing::default();
    style.spacing.scroll_bar_width = 20.0;
    style.spacing.button_padding = bevy_egui::egui::Vec2::new(10.0, 10.0);
    egui_context.ctx_mut().set_style(style);

    // Icons
    commands.insert_resource(SandboxGui {
        mode: GuiMode::MainGui,
        last_element: Element::Sand,
        bucket_icon_handle: add_icon(
            &mut egui_context,
            "icon_bucket",
            include_bytes!("../assets/icon_bucket.png"),
        ),
        icon_circle_handle: add_icon(
            &mut egui_context,
            "icon_circle",
            include_bytes!("../assets/icon_circle.png"),
        ),
        icon_square_handle: add_icon(
            &mut egui_context,
            "icon_square",
            include_bytes!("../assets/icon_square.png"),
        ),
        icon_pencil_handle: add_icon(
            &mut egui_context,
            "icon_pencil",
            include_bytes!("../assets/icon_pencil.png"),
        ),
        icon_spray_handle: add_icon(
            &mut egui_context,
            "icon_spray",
            include_bytes!("../assets/icon_spray.png"),
        ),
        icon_play_handle: add_icon(
            &mut egui_context,
            "icon_play",
            include_bytes!("../assets/icon_play.png"),
        ),
        icon_pause_handle: add_icon(
            &mut egui_context,
            "icon_pause",
            include_bytes!("../assets/icon_pause.png"),
        ),
        icon_zoom_in_handle: add_icon(
            &mut egui_context,
            "icon_zoom_in",
            include_bytes!("../assets/icon_zoom_in.png"),
        ),
        icon_zoom_out_handle: add_icon(
            &mut egui_context,
            "icon_zoom_out",
            include_bytes!("../assets/icon_zoom_out.png"),
        ),
        icon_move_handle: add_icon(
            &mut egui_context,
            "icon_move",
            include_bytes!("../assets/icon_move.png"),
        ),
        icon_settings_handle: add_icon(
            &mut egui_context,
            "icon_settings",
            include_bytes!("../assets/icon_settings.png"),
        ),
        icon_empty_handle: add_icon(
            &mut egui_context,
            "icon_empty",
            include_bytes!("../assets/icon_empty.png"),
        ),
        icon_eraser_handle: add_icon(
            &mut egui_context,
            "icon_eraser",
            include_bytes!("../assets/icon_eraser.png"),
        ),
    });
}

fn add_icon(egui_context: &mut EguiContext, name: &str, image_data: &[u8]) -> TextureHandle {
    let image = image::load_from_memory(image_data).unwrap();
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    let icon_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
    let icon_texture_handle =
        egui_context
            .ctx_mut()
            .load_texture(name, icon_image, Default::default());
    icon_texture_handle
}
