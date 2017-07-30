use sfml::graphics::*;
use sfml::system::Vector2u;

use resources::Resources;

pub struct Bullet<'s> {
    sprite: Sprite<'s>
}

impl<'s> Bullet<'s> {
    fn new(res: &'s Resources, win_size: &Vector2u) -> Bullet<'s> {
        let mut sprite = Sprite::with_texture(res.bullet());
        sprite.set_scale2f(win_size.x as f32 / 1600., win_size.y as f32 / 1200.);
        
        Bullet {
            sprite,
        }
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
    pub bullets: Vec<Bullet<'s>>
}

impl<'s> BulletManager<'s> {
    pub fn new(res: &'s Resources, win_size: &Vector2u) -> BulletManager<'s> {
        let mut bullets = Vec::new();
        bullets.push(Bullet::new(res, win_size));
        
        BulletManager {
            bullets,
        }
    }
}

/*
impl<'s> IntoIterator for BulletManager<'s> {
    type Item = Bullet<'s>;
    type IntoIter = ::std::vec::IntoIter<Bullet<'s>>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.bullets.into_iter()
    }
}
*/

/*
impl<'s> AsRef<Bullet<'s>> for BulletManager<'s> {
    fn as_ref(&self) -> &Bullet<'s> {
        self.bullets.as_ref()
    } 
}
*/
