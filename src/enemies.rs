use core::slice;
use rand;
use rand::distributions::{IndependentSample, Range};
use sfml::graphics::*;
use sfml::system::Vector2u;

use resources::Resources;

const MAX_ENEMY_NUMBER: u32 = 4;
const WIN_SIDE_PADDING: u32 = 64;
const SPEED_RANGE: (u32, u32) = (300, 550);

pub struct Enemy<'s> {
    sprite: Sprite<'s>,
    speed: u32,
    number: u32,
}

impl<'s> Enemy<'s> {
    fn new(res: &'s Resources) -> Enemy<'s> {
        let mut sprite = Sprite::with_texture(res.enemy());
        //sprite.set_scale2f(0.5, 0.5);
        
        Enemy {
            sprite,
            speed: 0,
            number: 0,
        }
    }
    
    fn update(&mut self, delta: f32, win_size: &Vector2u) {
        self.sprite.move2f(0., self.speed as f32 * delta);
        
        if self.sprite.position().y > win_size.y as f32 {
            let current_num = self.number;
            self.spawn(current_num, win_size);
        }
    }
    
    fn spawn(&mut self, number: u32, win_size: &Vector2u) {
        self.number = number;
        
        let bounds = self.sprite.global_bounds();
        let width = win_size.x - WIN_SIDE_PADDING - WIN_SIDE_PADDING;
        
        self.sprite.set_position2f((width as f32 / ((MAX_ENEMY_NUMBER - 1) as f32)) * (self.number as f32)
                                            + WIN_SIDE_PADDING as f32 + - bounds.width / 2.,
                                   -bounds.height);
        
        self.speed = Range::new(SPEED_RANGE.0, SPEED_RANGE.1).ind_sample(&mut rand::thread_rng());
    }
    
    pub fn bounds(&self) -> FloatRect {
        self.sprite.global_bounds()
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
    win_size: Vector2u,
}

impl<'s> EnemyManager<'s> {
    pub fn new(res: &'s Resources, win_size: &Vector2u) -> EnemyManager<'s> {
        let mut active = Vec::with_capacity(32);
        
        let mut test = Enemy::new(res);
        test.spawn(0, win_size);
        active.push(test);
        
        let mut test = Enemy::new(res);
        test.spawn(1, win_size);
        active.push(test);
        
        let mut test = Enemy::new(res);
        test.spawn(2, win_size);
        active.push(test);
        
        let mut test = Enemy::new(res);
        test.spawn(3, win_size);
        active.push(test);
        
        EnemyManager {
            active,
            inactive: Vec::with_capacity(32),
            win_size: win_size.clone(),
        }        
    }
    
    pub fn update(&mut self, delta: f32) {
        for i in &mut self.active {
            i.update(delta, &self.win_size);
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
