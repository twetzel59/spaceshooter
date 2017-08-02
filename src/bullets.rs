use core::slice;
use sfml::graphics::*;
use sfml::system::Vector2u;

use resources::Resources;

const SPEED: f32 = 1000.;

pub struct Bullet<'s> {
    sprite: Sprite<'s>
}

impl<'s> Bullet<'s> {
    fn new(res: &'s Resources, win_size: &Vector2u) -> Bullet<'s> {
        let mut sprite = Sprite::with_texture(res.bullet());
        sprite.set_scale2f(win_size.x as f32 / 3200., win_size.y as f32 / 2400.);
        sprite.set_position2f(0., win_size.y as f32 - res.bullet().size().y as f32);
        
        Bullet {
            sprite,
        }
    }
    
    fn update(&mut self, delta: f32) {
        self.sprite.move2f(0., -SPEED * delta);
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
    bullets: Vec<Bullet<'s>>
}

impl<'s> BulletManager<'s> {
    pub fn new(res: &'s Resources, win_size: &Vector2u) -> BulletManager<'s> {
        let mut bullets = Vec::new();
        bullets.push(Bullet::new(res, win_size));
        
        BulletManager {
            bullets,
        }
    }
    
    pub fn update(&mut self, delta: f32) {
        for i in &mut self.bullets {
            i.update(delta);
        }
    }
}

impl<'a, 's> IntoIterator for &'a BulletManager<'s> {
    type Item = &'a Bullet<'s>;
    type IntoIter = slice::Iter<'a, Bullet<'s>>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.bullets.iter()
    }
}
