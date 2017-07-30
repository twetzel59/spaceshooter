use sfml::graphics::{Texture, TextureRef};

pub struct Resources {
    background: Texture,
    ship: Texture,
    enemy: Texture,
    bullet: Texture,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            background: Texture::from_file("res/space.png").unwrap(),
            ship: Texture::from_file("res/ship.png").unwrap(),
            enemy: Texture::from_file("res/enemy.png").unwrap(),
            bullet: Texture::from_file("res/bullet.png").unwrap(),
        }
    }
    
    pub fn background(&self) -> &TextureRef {
        &self.background
    }
    
    pub fn ship(&self) -> &TextureRef {
        &self.ship
    }
    
    pub fn enemy(&self) -> &TextureRef {
        &self.enemy
    }
    
    pub fn bullet(&self) -> &TextureRef {
        &self.bullet
    }
}
