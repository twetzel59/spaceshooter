extern crate core;
extern crate rand;
extern crate sfml;

use sfml::graphics::{RenderWindow};
use sfml::window::{ContextSettings, style, VideoMode};

pub use attackable::Attackable;
pub use background::Background;
pub use resources::Resources;
pub use ship::Ship;
pub use scoreboard::Scoreboard;
pub use winstatus::WinStatus;

pub mod background;
pub mod bullets;
pub mod collision;
pub mod enemies;
pub mod resources;
pub mod scoreboard;
pub mod ship;
pub mod soundfilter;
pub mod winstatus;

mod attackable;

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
