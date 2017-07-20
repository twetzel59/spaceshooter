use sfml::graphics::{Drawable, RenderStates, RenderTarget, Sprite};

use resources::Resources;

pub struct Background<'s> {
    sprite: Sprite<'s>,
}

impl<'s> Background<'s> {
    pub fn new(res: &'s Resources) -> Background<'s> {
        Background {
            sprite: Sprite::with_texture(res.background()),
        }
    }
}

impl<'s> Drawable for Background<'s> {
    fn draw<'se, 'tex, 'sh, 'shte> (
        &'se self,
        target: &mut RenderTarget,
        states: RenderStates<'tex, 'sh, 'shte>
    )
        where 'se: 'sh {
        
        target.draw_sprite(&self.sprite, states);
    }
}
