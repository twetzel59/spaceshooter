use core::slice;
use sfml::graphics::*;
use sfml::system::Vector2u;

use attackable::*;
use resources::Resources;

const SPEED: f32 = 1000.;

pub struct Bullet<'s> {
    sprite: Sprite<'s>,
    dead: bool,
}

impl<'s> Bullet<'s> {
    fn new(res: &'s Resources, win_size: &Vector2u) -> Bullet<'s> {
        let mut sprite = Sprite::with_texture(res.bullet());
        sprite.set_scale2f(win_size.x as f32 / 1600., win_size.y as f32 / 1200.);
        //sprite.set_position2f(0., win_size.y as f32 - res.bullet().size().y as f32);
        
        Bullet {
            sprite,
            dead: false,
        }
    }
    
    fn update(&mut self, delta: f32) {
        self.sprite.move2f(0., -SPEED * delta);
    }
    
    fn shoot(&mut self, ship_bounds: &FloatRect) {
        //let height = self.sprite.texture().unwrap().size().y as f32;
        //self.sprite.set_position(from_pos);
        
        let width = self.sprite.global_bounds().width;
        
        self.sprite.set_position2f(ship_bounds.left + ship_bounds.width / 2. - width / 2., ship_bounds.top);
        
        self.dead = false;
    }
    
    pub fn bounds(&self) -> FloatRect {
        self.sprite.global_bounds()
    }
}

impl<'s> Attackable for Bullet<'s> {
    fn die(&mut self) {
        self.dead = true;
    }
    
    fn check(&self) -> Status {
        if self.dead || self.sprite.position().y < -(self.sprite.global_bounds().height as f32) {
            Status::Dead
        } else {
            Status::Alive
        }
    }
}

impl<'s> Drawable for Bullet<'s> {
    fn draw<'se, 'tex, 'sh, 'shte>(&'se self, 
                                   target: &mut RenderTarget,
                                   states: RenderStates<'tex, 'sh, 'shte>)
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
        let active = Vec::with_capacity(64);
        //active.push(Bullet::new(res, win_size));
        
        let mut inactive = Vec::with_capacity(64);
        for _ in 0..64 {
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
        let mut i: usize = 0;
        loop {
            if i >= self.active.len() {
                break;
            }
            
            self.active[i].update(delta);
            
            match self.active[i].check() {
                Status::Dead => {
                    self.inactive.push(self.active.remove(i));
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

impl<'a, 's> IntoIterator for &'a mut BulletManager<'s> {
    type Item = &'a mut Bullet<'s>;
    type IntoIter = slice::IterMut<'a, Bullet<'s>>;
    
    fn into_iter(mut self) -> Self::IntoIter {
        self.active.iter_mut()
    }
}
