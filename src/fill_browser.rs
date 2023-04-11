use bevy::{prelude::*, window::WindowResolution};

/// Plugin that matches the application window to fill the browser window.
/// Does nothing if not run in a browser (wasm).
pub struct FillBrowserWindowPlugin;

impl Plugin for FillBrowserWindowPlugin {
    #[allow(unreachable_code, unused_variables)]
    fn build(&self, app: &mut App) {
        #[cfg(not(target_family = "wasm"))]
        {
            return;
        }
        app.add_system(browser_filler);
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
