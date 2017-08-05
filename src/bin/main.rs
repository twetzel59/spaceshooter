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
    
    let mut bullet_mgr = bullets::BulletManager::new(&res, &size);
    let mut enemy_mgr = enemies::EnemyManager::new(&res, &size);
    
    let mut scoreboard = Scoreboard::new(&res, &size);
    
    let mut clock = Clock::start();
    
    let mut won: bool = false;
    'game: loop {
        let delta_t = clock.restart().as_seconds();
        
        if collision::handle_collisions(&mut enemy_mgr, &mut bullet_mgr) {
            scoreboard.score();
        }
        bullet_mgr.update(delta_t);
        match enemy_mgr.update(delta_t) {
            WinStatus::Won => {
                won = true;
                break 'game;
            }
            _ => {},
        }
        
        win.draw(&back);
        for i in &bullet_mgr {
            win.draw(i);
        }
        for i in &enemy_mgr {
            win.draw(i);
        }
        win.draw(&ship);
        win.draw(&scoreboard);
        win.display();
        
        while let Some(e) = win.poll_event() {
            match e {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } =>
                    break 'game,
                //Event::KeyPressed { code: Key::Left, .. } => ship.on_left(delta_t),
                //Event::KeyPressed { code: Key::Right, .. } => ship.on_right(delta_t),
                Event::KeyPressed { code: Key::Space, .. } => {
                    bullet_mgr.shoot(&ship.bounds());
                },
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
    
    if won {
        scoreboard.show_win();
        
        'scorescreen: loop {
            win.draw(&back);
            win.draw(&scoreboard);
            win.display();
            
            while let Some(e) = win.poll_event() {
                match e {
                    Event::Closed | Event::KeyPressed { code: Key::Escape, .. } =>
                        break 'scorescreen,
                    _ => {},
                }
            }
        }
    }
}
