use core::slice;
use rand;
use rand::distributions::{IndependentSample, Range};
use sfml::audio::Sound;
use sfml::graphics::*;
use sfml::system::Vector2u;

use attackable::*;
use resources::Resources;
use soundfilter;
use winstatus::WinStatus;

const MAX_ENEMY_NUMBER: u32 = 8;
const WIN_SIDE_PADDING: u32 = 64;
const SPEED_RANGE: (u32, u32) = (300, 550);

pub struct Enemy<'s> {
    sprite: Sprite<'s>,
    speed: u32,
    number: u32,
    dead: bool,
}

#[derive(Clone, Copy)]
struct Arrangement {
    my_num: u32,
    total: u32,
}

impl<'s> Enemy<'s> {
    fn new(res: &'s Resources, arrangement: Arrangement, win_size: &Vector2u) -> Enemy<'s> {
        let mut e = Enemy {
            sprite: Sprite::with_texture(res.enemy()),
            speed: 0,
            number: 0,
            dead: false,
        };
        
        e.spawn(arrangement, win_size);
        
        e
    }
    
    fn update(&mut self, delta: f32, win_size: &Vector2u) {
        self.sprite.move2f(0., self.speed as f32 * delta);
        
        if self.sprite.position().y > win_size.y as f32 {
            //let current_num = self.number;
            self.go_to_top();
            //self.spawn(current_num, win_size);
        }
    }
    
    fn go_to_top(&mut self) {
        let bounds = self.sprite.global_bounds();
        self.sprite.set_position2f(bounds.left, -bounds.height);
    }
    
    fn spawn(&mut self, arrangement: Arrangement, win_size: &Vector2u) {
        self.number = arrangement.my_num;
        
        let scale = 4. / arrangement.total as f32;
        self.sprite.set_scale2f(scale, scale);
        
        let bounds = self.sprite.global_bounds();
        let width = win_size.x - WIN_SIDE_PADDING - WIN_SIDE_PADDING;
        
        self.sprite.set_position2f((width as f32 / ((arrangement.total - 1) as f32)) * (self.number as f32)
                                            + WIN_SIDE_PADDING as f32 + - bounds.width / 2.,
                                   -bounds.top);
        self.go_to_top();
        
        self.speed = Range::new(SPEED_RANGE.0, SPEED_RANGE.1).ind_sample(&mut rand::thread_rng());
        
        self.dead = false;
    }
    
    pub fn bounds(&self) -> FloatRect {
        self.sprite.global_bounds()
    }
}

impl<'s> Attackable for Enemy<'s> {
    fn die(&mut self) {
        self.dead = true;
    }
    
    fn check(&self) -> Status {
        if self.dead {
            Status::Dead
        } else {
            Status::Alive
        }
    }
}

impl<'s> Drawable for Enemy<'s> {
    fn draw<'se, 'tex, 'sh, 'shte>(&'se self, 
                                   target: &mut RenderTarget,
                                   states: RenderStates<'tex, 'sh, 'shte>)
        where 'se: 'sh
    {
        target.draw_sprite(&self.sprite, states);
    }
}

pub struct EnemyManager<'s> {
    active: Vec<Enemy<'s>>,
    inactive: Vec<Enemy<'s>>,
    win_size: Vector2u,
    kill_sound: Sound<'s>,
    wave: u32
}

impl<'s> EnemyManager<'s> {
    pub fn new(res: &'s Resources, win_size: &Vector2u) -> EnemyManager<'s> {
        /*
        let wave = 2;
        for i in 0..wave {
            active.push(Enemy::new(res, Arrangement { my_num: i, total: wave }, win_size));
        }
        */
        
        let mut inactive = Vec::with_capacity(32);
        
        for i in 0..MAX_ENEMY_NUMBER {
            inactive.push(Enemy::new(res, Arrangement { my_num: i, total: MAX_ENEMY_NUMBER }, win_size));
        }
        
        let mut m = EnemyManager {
            active: Vec::with_capacity(MAX_ENEMY_NUMBER as usize),
            inactive,
            win_size: win_size.clone(),
            kill_sound: Sound::with_buffer(res.kill()),
            wave: 1,
        };
        
        m.start_wave();
        
        m
    }
    
    pub fn update(&mut self, delta: f32) -> WinStatus {
        let mut i: usize = 0;
        loop {
            if i >= self.active.len() {
                break;
            }
            
            self.active[i].update(delta, &self.win_size);
            
            match self.active[i].check() {
                Status::Dead => {
                    self.inactive.push(self.active.remove(i));
                    
                    soundfilter::filter_random(&mut self.kill_sound);
                    self.kill_sound.play();
                },
                _ => {},
            }
            
            i += 1;
        }
        
        if self.active.len() == 0 {
            if self.wave < MAX_ENEMY_NUMBER {
                self.start_wave();
            } else {
                return WinStatus::Won;
            }
        }
        
        WinStatus::Playing
    }
    
    fn start_wave(&mut self) {
        self.wave += 1;
        
        for i in 0..self.wave {
            let mut enemy = self.inactive.pop().unwrap();
            enemy.spawn(Arrangement { my_num: i, total: self.wave }, &self.win_size);
            
            self.active.push(enemy);
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

impl<'a, 's> IntoIterator for &'a mut EnemyManager<'s> {
    type Item = &'a mut Enemy<'s>;
    type IntoIter = slice::IterMut<'a, Enemy<'s>>;
    
    fn into_iter(mut self) -> Self::IntoIter {
        self.active.iter_mut()
    }
}
