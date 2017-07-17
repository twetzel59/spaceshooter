use sfml::graphics::{Texture};

pub struct Resources {
    background: Texture,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            background: Texture::from_file("res/space.png").unwrap(),
        }
    }
    
    pub fn background(&self) -> &Texture {
        &self.background
    }
}
