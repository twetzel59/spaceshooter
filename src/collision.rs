//use sfml::graphics::FloatRect;

use attackable::Attackable;
use enemies::*;
use bullets::*;
use ship::*;

pub fn handle_bullets(enemies: &mut EnemyManager, bullets: &mut BulletManager) -> bool {
    let mut hit = false;
    
    for e in &mut *enemies {
        for b in &mut *bullets {
            if let Some(_) = e.bounds().intersection(&b.bounds()) {
                hit = true;
                e.die();
                b.die();
            }
        }
    }
    
    hit
    
    /*
    let mut dead_enemies: Vec<usize> = Vec::new();
    let mut dead_bullets: Vec<usize> = Vec::new();
    
    for (ie, e) in (&*enemies).into_iter().enumerate() {
        for (ib, b) in (&*bullets).into_iter().enumerate() {
            if let Some(_) = e.bounds().intersection(&b.bounds()) {
                dead_enemies.push(ie);
                dead_bullets.push(ib);
            }
        }
    }
    
    enemies.register_dead(&dead_enemies);
    bullets.register_dead(&dead_bullets);
    */
}

pub fn handle_ship(ship: &Ship, enemies: &EnemyManager) -> bool {
    for e in enemies {
        if let Some(_) = e.bounds().intersection(&ship.bounds()) {
            return true;
        }
    }
    
    false
}
