use core::slice;
use sfml::graphics::*;
use sfml::system::Vector2u;

use resources::Resources;

pub struct Enemy<'s> {
    sprite: Sprite<'s>,
}

impl<'s> Enemy<'s> {
    fn new(res: &'s Resources, win_size: &Vector2u) -> Enemy<'s> {
        let mut sprite = Sprite::with_texture(res.enemy());
        //sprite.set_scale2f(0.5, 0.5);
        
        Enemy {
            sprite,
        }
    }
    
    fn update(&mut self, delta: f32) {
        
    }
}

impl<'s> Drawable for Enemy<'s> {
    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self, 
        target: &mut RenderTarget, 
        states: RenderStates<'tex, 'sh, 'shte>
    )
        where 'se: 'sh {
        
        target.draw_sprite(&self.sprite, states);
    }
}

pub struct EnemyManager<'s> {
    active: Vec<Enemy<'s>>,
    inactive: Vec<Enemy<'s>>,
}

impl<'s> EnemyManager<'s> {
    pub fn new(res: &'s Resources, win_size: &Vector2u) -> EnemyManager<'s> {
        let mut active = Vec::with_capacity(32);
        active.push(Enemy::new(res, win_size));
        
        EnemyManager {
            active,
            inactive: Vec::with_capacity(32),
        }        
    }
    
    pub fn update(&mut self, delta: f32) {
        for i in &mut self.active {
            i.update(delta);
        }
    }
}

impl<'a, 's> IntoIterator for &'a EnemyManager<'s> {
    type Item = &'a Enemy<'s>;
    type IntoIter = slice::Iter<'a, Enemy<'s>>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.active.iter()
    }
}
