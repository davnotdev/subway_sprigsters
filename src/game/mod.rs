use super::*;

mod consts;
mod subway;

use consts::*;
use subway::*;

pub enum Game {
    Subway(SubwayLevel),
}

impl Game {
    pub fn new() -> Self {
        Self::Subway(SubwayLevel::new())
    }

    pub fn update(&mut self, buttons: Buttons) {
        match self {
            Game::Subway(level) => level.update(buttons),
        }
    }

    pub fn render(&mut self, fb: &mut Framebuffer) {
        match self {
            Game::Subway(level) => level.render(fb),
        }
    }
}
