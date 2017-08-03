use core::slice;
use sfml::graphics::*;
use sfml::system::{Vector2f, Vector2u};

use resources::Resources;

const SPEED: f32 = 1000.;

#[derive(Clone, Copy)]
enum Status {
    Onscreen,
    Offscreen,
}

pub struct Bullet<'s> {
    sprite: Sprite<'s>
}

impl<'s> Bullet<'s> {
    fn new(res: &'s Resources, win_size: &Vector2u) -> Bullet<'s> {
        let mut sprite = Sprite::with_texture(res.bullet());
        sprite.set_scale2f(win_size.x as f32 / 3200., win_size.y as f32 / 2400.);
        //sprite.set_position2f(0., win_size.y as f32 - res.bullet().size().y as f32);
        
        Bullet {
            sprite,
        }
    }
    
    fn update(&mut self, delta: f32) {
        self.sprite.move2f(0., -SPEED * delta);
    }
    
    fn check(&self) -> Status {
        if self.sprite.position().y < -(self.sprite.global_bounds().height as f32) {
            Status::Offscreen
        } else {
            Status::Onscreen
        }
    }
    
    fn shoot(&mut self, ship_bounds: &FloatRect) {
        //let height = self.sprite.texture().unwrap().size().y as f32;
        //self.sprite.set_position(from_pos);
        
        let width = self.sprite.global_bounds().width;
        
        self.sprite.set_position2f(ship_bounds.left + ship_bounds.width / 2. - width / 2., ship_bounds.top);
    }
}

impl<'s> Drawable for Bullet<'s> {
    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self, 
        target: &mut RenderTarget, 
        states: RenderStates<'tex, 'sh, 'shte>
    )
        where 'se: 'sh {
        
        target.draw_sprite(&self.sprite, states);
    }
}

pub struct BulletManager<'s> {
    active: Vec<Bullet<'s>>,
    inactive: Vec<Bullet<'s>>,
}

impl<'s> BulletManager<'s> {
    pub fn new(res: &'s Resources, win_size: &Vector2u) -> BulletManager<'s> {
        let mut active = Vec::with_capacity(64);
        //active.push(Bullet::new(res, win_size));
        
        let mut inactive = Vec::with_capacity(64);
        for i in 0..64 {
            /*
            let mut b = Bullet::new(res, win_size);
            b.sprite.move2f((res.bullet().size().x * i) as f32, 0.);
            
            inactive.push(b);
            */
            
            inactive.push(Bullet::new(res, win_size));
        }
        
        BulletManager {
            active,
            inactive,
        }
    }
    
    pub fn update(&mut self, delta: f32) {
        for i in &mut self.active {
            i.update(delta);
        }
        
        let mut i: usize = 0;
        loop {
            if i >= self.active.len() {
                break;
            }
            
            match self.active[i].check() {
                Status::Offscreen => {
                    let bullet = self.active.remove(i);
                    
                    self.inactive.push(bullet);
                },
                _ => {},
            }
            
            i += 1;
        }
    }
    
    pub fn shoot(&mut self, ship_bounds: &FloatRect) {
        if let Some(mut bullet) = self.inactive.pop() {
            bullet.shoot(ship_bounds);
            
            self.active.push(bullet);
        }
    }
}

impl<'a, 's> IntoIterator for &'a BulletManager<'s> {
    type Item = &'a Bullet<'s>;
    type IntoIter = slice::Iter<'a, Bullet<'s>>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.active.iter()
    }
}
