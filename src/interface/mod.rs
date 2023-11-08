use bevy::prelude::*;
use crate::interface::gui::GuiPlugin;
use crate::interface::pointer_input::PointerInputPlugin;
use crate::interface::toolbox::ToolBox;

mod fill_browser;
mod gui;
mod pointer_input;
mod toolbox;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GuiPlugin)
            .add_plugins(PointerInputPlugin)
            .init_resource::<ToolBox>();

        #[cfg(target_family = "wasm")]
        app.add_plugins(FillBrowserWindowPlugin);
    }
}