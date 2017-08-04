//use sfml::graphics::{Drawable, RenderStates, RenderTarget, Sprite};
use sfml::graphics::*;
use sfml::system::Vector2u;

use resources::Resources;

const SPEED: f32 = 728.;

pub struct Ship<'s> {
    sprite: Sprite<'s>,
    left_max: f32,
    right_max: f32,
}

impl<'s> Ship<'s> {
    pub fn new(res: &'s Resources, win_size: &Vector2u) -> Ship<'s> {
        let mut sprite = Sprite::with_texture(res.ship());
        sprite.set_position2f((win_size.x / 2 - res.ship().size().x / 2) as f32,
                              win_size.y as f32 / 32. * 31. - res.ship().size().y as f32);
        
        Ship {
            sprite,
            left_max: win_size.x as f32 / 32.,
            right_max: win_size.x as f32 / 32. * 31. - res.ship().size().x as f32,
        }
    }
    
    /*
    pub fn update(&mut self) {
        let pos = self.sprite.position();
        self.sprite.set_position2f(self.right_max, pos.y);
    }
    */
    
    pub fn on_left(&mut self, delta_t: f32) {
        self.sprite.move2f(-SPEED * delta_t, 0.);
        
        if self.sprite.position().x < self.left_max {
            let pos = self.sprite.position();
            self.sprite.set_position2f(self.left_max, pos.y);
        }
    }
    
    pub fn on_right(&mut self, delta_t: f32) {
        self.sprite.move2f(SPEED * delta_t, 0.);
        
        if self.sprite.position().x > self.right_max {
            let pos = self.sprite.position();
            self.sprite.set_position2f(self.right_max, pos.y);
        }
    }
    
    pub fn bounds(&self) -> FloatRect {
        self.sprite.global_bounds()
    }
}

impl<'s> Drawable for Ship<'s> {
    fn draw<'se, 'tex, 'sh, 'shte>(&'se self, 
                                   target: &mut RenderTarget,
                                   states: RenderStates<'tex, 'sh, 'shte>)
        where 'se: 'sh
    {
        target.draw_sprite(&self.sprite, states);
    }
}
