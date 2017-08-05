use sfml::graphics::*;
use sfml::system::Vector2u;

use resources::Resources;

const SCORE_AMOUNT: u64 = 10_000;

pub struct Scoreboard<'s> {
    score: u64,
    text: Text<'s>,
}

impl<'s> Scoreboard<'s> {
    pub fn new(res: &'s Resources, win_size: &Vector2u) -> Scoreboard<'s> {
        let mut text = Text::new_init("0", res.font(), 24);
        text.set_position2f(win_size.x as f32 / 20., win_size.y as f32 / 25.);
        
        Scoreboard {
            score: 0,
            text,
        }
    }
    
    pub fn score(&mut self) {
        self.score += SCORE_AMOUNT;
        self.render();
    }
    
    pub fn show_win(&mut self) {
        self.text.set_fill_color(&Color::rgb(255, 100, 16));
        self.text.set_character_size(26);
    }
    
    pub fn show_loss(&mut self) {
        self.text.set_fill_color(&Color::rgb(16, 100, 255));
        self.text.set_character_size(26);
    }
    
    fn render(&mut self) {
        self.text.set_string(&format!("{}", self.score));
    }
}

impl<'s> Drawable for Scoreboard<'s> {
    fn draw<'se, 'tex, 'sh, 'shte>(&'se self, 
                                   target: &mut RenderTarget,
                                   states: RenderStates<'tex, 'sh, 'shte>)
        
        where 'se: 'sh
    {
        target.draw_text(&self.text, states);
    }
}
