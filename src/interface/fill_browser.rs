use bevy::{prelude::*, window::WindowResolution};

/// Plugin that matches the application window to fill the browser window. Only useable for wasm targets.
pub struct FillBrowserWindowPlugin;

impl Plugin for FillBrowserWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, browser_filler);
    }
}

fn browser_filler(mut window: Query<&mut Window>) {
    // Get browser window inner size
    let browser_window = web_sys::window().unwrap();
    let browser_width = browser_window.inner_width().unwrap().as_f64().unwrap() as u32;
    let browser_height = browser_window.inner_height().unwrap().as_f64().unwrap() as u32;

    // Set it as our application window size (this will do nothing if it is the same as previous)
    let mut window = window.get_single_mut().unwrap();
    window.resolution = WindowResolution::new(browser_width as f32, browser_height as f32);
}
