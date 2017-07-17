extern crate spaceshooter;
extern crate sfml;

use spaceshooter::*;

use sfml::graphics::*;
use sfml::window::{Event, Key};

fn main() {
    let mut win = create_window();
    
    let res = Resources::new();
    
    let back = Background::new(&res);
    
    'game: loop {
        win.clear(&Color::black());
        win.draw(&back);
        win.display();
        
        while let Some(e) = win.poll_event() {
            match e {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } =>
                    break 'game,
                _ => {},
            }
        }
    }
}
