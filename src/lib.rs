extern crate sfml;

use sfml::graphics::{RenderWindow};
use sfml::window::{ContextSettings, style, VideoMode};

pub use background::Background;
pub use resources::Resources;

pub mod background;
pub mod resources;

pub fn create_window() -> RenderWindow {
    let mut win = RenderWindow::new(VideoMode::new(800, 600, 32), "rsPong",
                                style::TITLEBAR | style::CLOSE,
                                &ContextSettings::default())
                            .unwrap();
                            
    win.set_vertical_sync_enabled(true);
    
    win
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
