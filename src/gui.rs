use bevy::prelude::*;
use bevy_egui::{
    egui::{self, style::*, Color32, ColorImage, Frame, Layout, TextureHandle, Ui},
    EguiContext, EguiPlugin,
};

use crate::{
    element::*,
    render::cell_color,
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
    pub element_icons: [TextureHandle; MAX_ELEMENT_ID as usize],
}

pub fn gui_system(
    mut egui_context: ResMut<EguiContext>,
    camera: Query<&mut Transform, With<Camera>>,
    mut gui: ResMut<SandboxGui>,
    mut toolbox: ResMut<ToolBox>,
    mut simulation: ResMut<Simulation>,
    mut sandbox: Query<(Entity, &mut SandBox)>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    egui::SidePanel::right("right_panel")
        .frame(Frame::none())
        .show_separator_line(false)
        .resizable(false)
        .min_width(64.0)
        .show(egui_context.ctx_mut(), |ui| {
            let settings_button =
                egui::widgets::ImageButton::new(&gui.icon_settings_handle, [64.0, 64.0])
                    .frame(false);
            let settings_button = if gui.mode == GuiMode::SandboxSettings {
                settings_button.tint(Color32::LIGHT_GREEN)
            } else {
                settings_button
            };
            if ui.add(settings_button).clicked() {
                gui.mode = if gui.mode == GuiMode::SandboxSettings {
                    GuiMode::MainGui
                } else {
                    GuiMode::SandboxSettings
                }
            };
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

            view_gui(ui, gui.as_mut(), camera);
        });

    egui::TopBottomPanel::bottom("bottom_panel")
        .frame(Frame::none())
        .show_separator_line(false)
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                tool_gui(ui, gui.as_mut(), toolbox.as_mut());
            });
        });

    if gui.mode == GuiMode::SandboxSettings {
        egui::CentralPanel::default()
            .frame(Frame::none())
            .show(egui_context.ctx_mut(), |ui| {
                ui.with_layout(
                    Layout::from_main_dir_and_cross_align(
                        egui::Direction::LeftToRight,
                        egui::Align::Min,
                    )
                    .with_main_wrap(true),
                    |ui| {
                        let (entity, mut sandbox) = sandbox.single_mut();
                        if !simulation.running {
                            if ui.button("Step").clicked() {
                                simulation.step = true;
                            }
                        }
                        ui.label(format!("Simulation: {} ms", simulation.frame_time_ms));
                        ui.label(format!("Render: {} ms", sandbox.render_time_ms));
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
    } else if gui.mode == GuiMode::ElementSelect {
        egui::CentralPanel::default()
            .frame(Frame::none())
            .show(egui_context.ctx_mut(), |ui| {
                ui.with_layout(
                    Layout::from_main_dir_and_cross_align(
                        egui::Direction::LeftToRight,
                        egui::Align::Min,
                    )
                    .with_main_wrap(true),
                    |ui| {
                        element_button(ui, &mut gui, Element::Sand, &mut toolbox);
                        element_button(ui, &mut gui, Element::Wood, &mut toolbox);
                        element_button(ui, &mut gui, Element::Iron, &mut toolbox);
                        element_button(ui, &mut gui, Element::Rock, &mut toolbox);
                        element_button(ui, &mut gui, Element::Water, &mut toolbox);
                        element_button(ui, &mut gui, Element::Acid, &mut toolbox);
                        element_button(ui, &mut gui, Element::Oil, &mut toolbox);
                        element_button(ui, &mut gui, Element::Lava, &mut toolbox);
                        element_button(ui, &mut gui, Element::Fire, &mut toolbox);
                        element_button(ui, &mut gui, Element::Life, &mut toolbox);
                        element_button(ui, &mut gui, Element::Seed, &mut toolbox);
                        element_button(ui, &mut gui, Element::TNT, &mut toolbox);
                        element_button(ui, &mut gui, Element::Fuse, &mut toolbox);
                        element_button(ui, &mut gui, Element::WaterSource, &mut toolbox);
                        element_button(ui, &mut gui, Element::AcidSource, &mut toolbox);
                        element_button(ui, &mut gui, Element::LavaSource, &mut toolbox);
                        element_button(ui, &mut gui, Element::FireSource, &mut toolbox);
                        element_button(ui, &mut gui, Element::Drain, &mut toolbox);
                    },
                );
            });
    }
    if gui.mode == GuiMode::ToolSelect {
        egui::CentralPanel::default()
            .frame(Frame::none())
            .show(egui_context.ctx_mut(), |ui| {
                ui.with_layout(
                    Layout::from_main_dir_and_cross_align(
                        egui::Direction::LeftToRight,
                        egui::Align::Min,
                    )
                    .with_main_wrap(true),
                    |ui| {
                        if ui
                            .add(
                                egui::widgets::ImageButton::new(
                                    &gui.icon_pencil_handle,
                                    [64.0, 64.0],
                                )
                                .frame(false),
                            )
                            .clicked()
                        {
                            toolbox.tool = Tool::Pixel;
                            gui.mode = GuiMode::MainGui;
                        };
                        if ui
                            .add(
                                egui::widgets::ImageButton::new(
                                    &gui.icon_circle_handle,
                                    [64.0, 64.0],
                                )
                                .frame(false),
                            )
                            .clicked()
                        {
                            toolbox.tool = Tool::Circle;
                            gui.mode = GuiMode::MainGui;
                        };
                        if ui
                            .add(
                                egui::widgets::ImageButton::new(
                                    &gui.icon_square_handle,
                                    [64.0, 64.0],
                                )
                                .frame(false),
                            )
                            .clicked()
                        {
                            toolbox.tool = Tool::Square;
                            gui.mode = GuiMode::MainGui;
                        };
                        if ui
                            .add(
                                egui::widgets::ImageButton::new(
                                    &gui.icon_spray_handle,
                                    [64.0, 64.0],
                                )
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
                    },
                );
            });
    }
}

fn element_button(
    ui: &mut Ui,
    gui: &mut ResMut<SandboxGui>,
    element: Element,
    toolbox: &mut ResMut<ToolBox>,
) {
    if ui
        .add(
            egui::widgets::ImageButton::new(&gui.element_icons[element as usize], [64.0, 64.0])
                .frame(true),
        )
        .clicked()
    {
        toolbox.element = element;
        gui.mode = GuiMode::MainGui;
    };
}

fn view_gui(ui: &mut Ui, gui: &mut SandboxGui, mut camera: Query<&mut Transform, With<Camera>>) {
    if ui
        .add(egui::widgets::ImageButton::new(&gui.icon_zoom_in_handle, [64.0, 64.0]).frame(false))
        .clicked()
    {
        let mut transform = camera.single_mut();
        transform.scale.x = (transform.scale.x * 0.9).clamp(0.1, 1.0);
        transform.scale.y = (transform.scale.y * 0.9).clamp(0.1, 1.0);
    };
    if ui
        .add(egui::widgets::ImageButton::new(&gui.icon_zoom_out_handle, [64.0, 64.0]).frame(false))
        .clicked()
    {
        let mut transform = camera.single_mut();
        transform.scale.x = (transform.scale.x * 1.1).clamp(0.1, 1.0);
        transform.scale.y = (transform.scale.y * 1.1).clamp(0.1, 1.0);
    };
    let move_button =
        egui::widgets::ImageButton::new(&gui.icon_move_handle, [64.0, 64.0]).frame(false);
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
}

fn tool_gui(ui: &mut Ui, gui: &mut SandboxGui, toolbox: &mut ToolBox) {
    let eraser_button =
        egui::widgets::ImageButton::new(&gui.icon_eraser_handle, [64.0, 64.0]).frame(false);
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
}

fn element_gui(ui: &mut Ui, gui: &mut SandboxGui, toolbox: &mut ToolBox) {
    let element_button =
        egui::widgets::ImageButton::new(&gui.element_icons[toolbox.element as usize], [64.0, 64.0])
            .frame(false);
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

fn setup_gui(mut commands: Commands, mut egui_context: ResMut<EguiContext>) {
    // General styling
    let mut style = egui::Style::default();
    style.spacing = Spacing::default();
    style.spacing.scroll_bar_width = 20.0;
    style.spacing.button_padding = bevy_egui::egui::Vec2::new(10.0, 10.0);
    egui_context.ctx_mut().set_style(style);

    // Generate element icons
    let element_icons = [
        generate_element_image(Element::Air, "Air", egui_context.as_mut()),
        generate_element_image(Element::Sand, "Sand", egui_context.as_mut()),
        generate_element_image(Element::Rock, "Rock", egui_context.as_mut()),
        generate_element_image(Element::Water, "Water", egui_context.as_mut()),
        generate_element_image(Element::Acid, "Acid", egui_context.as_mut()),
        generate_element_image(Element::Drain, "Drain", egui_context.as_mut()),
        generate_element_image(Element::Wood, "Wood", egui_context.as_mut()),
        generate_element_image(Element::Iron, "Iron", egui_context.as_mut()),
        generate_element_image(Element::Rust, "Rust", egui_context.as_mut()),
        generate_element_image(Element::Fire, "Fire", egui_context.as_mut()),
        generate_element_image(Element::Ash, "Ash", egui_context.as_mut()),
        generate_element_image(Element::Oil, "Oil", egui_context.as_mut()),
        generate_element_image(Element::Lava, "Lava", egui_context.as_mut()),
        generate_element_image(Element::Smoke, "Smoke", egui_context.as_mut()),
        generate_element_image(Element::Life, "Life", egui_context.as_mut()),
        generate_element_image(Element::Seed, "Seed", egui_context.as_mut()),
        generate_element_image(Element::Plant, "Plant", egui_context.as_mut()),
        generate_element_image(Element::TNT, "TNT", egui_context.as_mut()),
        generate_element_image(Element::Fuse, "Fuse", egui_context.as_mut()),
        generate_element_image(Element::Explosion, "Explosion", egui_context.as_mut()),
        generate_element_image(Element::WaterSource, "WaterSource", egui_context.as_mut()),
        generate_element_image(Element::AcidSource, "AcidSource", egui_context.as_mut()),
        generate_element_image(Element::OilSource, "OilSource", egui_context.as_mut()),
        generate_element_image(Element::FireSource, "FireSource", egui_context.as_mut()),
        generate_element_image(
            Element::Indestructible,
            "Indestructible",
            egui_context.as_mut(),
        ),
    ];

    //  Sand = 1,
    // Rock = 2,
    // Water = 3,
    // Acid = 4,
    // Drain = 5,
    // Wood = 6,
    // Iron = 7,
    // Rust = 8,
    // Fire = 9,
    // Ash = 10,
    // Oil = 11,
    // Lava = 12,
    // Smoke = 13,
    // Life = 14,
    // Seed = 15,
    // Plant = 16,
    // TNT = 17,
    // Fuse = 18,
    // Explosion = 19,
    // WaterSource = 20,
    // AcidSource = 21,
    // OilSource = 22,
    // FireSource = 23,
    // LavaSource = 24,
    // Indestructible = 25,

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
        element_icons,
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

pub fn generate_element_image(
    element: Element,
    name: &str,
    egui_context: &mut EguiContext,
) -> TextureHandle {
    let size = 64;
    let mut sandbox = SandBox::new(size, size, None);
    let mut toolbox = ToolBox::default();
    toolbox.element = element;
    toolbox.tool = Tool::Circle;
    toolbox.tool_size = size - 2;
    toolbox.apply(&mut sandbox, size / 2, size / 2);

    let mut img = ColorImage::new([size, size], Color32::TRANSPARENT);
    for y in 0..size {
        for x in 0..size {
            let cell = sandbox.get(x, y);
            let (r, g, b) = cell_color(cell);
            img[(x, y)] = Color32::from_rgba_premultiplied(
                r,
                g,
                b,
                if cell.element == Element::Air || cell.element == Element::Indestructible {
                    0
                } else {
                    255
                },
            );
        }
    }

    egui_context
        .ctx_mut()
        .load_texture(name, img, Default::default())
}
