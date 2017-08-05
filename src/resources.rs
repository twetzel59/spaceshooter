use sfml::audio::{SoundBuffer, SoundBufferRef};
use sfml::graphics::{Font, Texture, TextureRef};

pub struct Resources {
    background: Texture,
    ship: Texture,
    enemy: Texture,
    bullet: Texture,
    font: Font,
    shoot: SoundBuffer,
    kill: SoundBuffer,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            background: Texture::from_file("res/space.png").unwrap(),
            ship: Texture::from_file("res/ship.png").unwrap(),
            enemy: Texture::from_file("res/enemy.png").unwrap(),
            bullet: Texture::from_file("res/bullet.png").unwrap(),
            font: Font::from_file("res/UbuntuMono-B.ttf").unwrap(),
            shoot: SoundBuffer::from_file("res/shoot.flac").unwrap(),
            kill: SoundBuffer::from_file("res/kill.flac").unwrap(),
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
    
    pub fn font(&self) -> &Font {
        &self.font
    }
    
    pub fn shoot(&self) -> &SoundBufferRef {
        &self.shoot
    }
    
    pub fn kill(&self) -> &SoundBufferRef {
        &self.kill
    }
}
