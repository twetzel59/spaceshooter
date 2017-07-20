extern crate spaceshooter;
extern crate sfml;

use spaceshooter::*;

use sfml::graphics::*;
use sfml::system::Clock;
use sfml::window::{Event, Key};

fn main() {
    let mut win = create_window();
    let size = win.size();
    
    let res = Resources::new();
    
    let back = Background::new(&res);
    
    let mut ship = Ship::new(&res, &size);
    
    let mut clock = Clock::start();
    'game: loop {
        let delta_t = clock.restart().as_seconds();
        
        win.clear(&Color::black());
        win.draw(&back);
        win.draw(&ship);
        win.display();
        
        while let Some(e) = win.poll_event() {
            match e {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } =>
                    break 'game,
                //Event::KeyPressed { code: Key::Left, .. } => ship.on_left(delta_t),
                //Event::KeyPressed { code: Key::Right, .. } => ship.on_right(delta_t),
                _ => {},
            }
        }
        
        if Key::Left.is_pressed() {
            ship.on_left(delta_t);
        } else if Key::Right.is_pressed() {
            ship.on_right(delta_t);
        }
        
        //ship.update();
    }
}
