extern crate core;
extern crate rand;
extern crate sfml;

use sfml::graphics::{RenderWindow};
use sfml::window::{ContextSettings, style, VideoMode};

pub use background::Background;
pub use resources::Resources;
pub use ship::Ship;

pub mod background;
pub mod resources;
pub mod ship;
pub mod enemies;
pub mod bullets;
pub mod collision;

pub fn create_window() -> RenderWindow {
    let mut win = RenderWindow::new(VideoMode::new(800, 600, 32), "rsSpaceShooter",
                                style::TITLEBAR | style::CLOSE,
                                &ContextSettings::default())
                            .unwrap();
                            
    win.set_vertical_sync_enabled(true);
    win.set_key_repeat_enabled(false);
    
    win
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
