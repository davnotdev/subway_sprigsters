use super::*;

mod consts;
mod seed_screen;
mod subway;

use consts::*;
use seed_screen::*;
use subway::*;

pub enum Game {
    Subway(SubwayLevel),
    SeedScreen(SeedScreen),
}

impl Game {
    pub fn new() -> Self {
        Self::SeedScreen(SeedScreen::new())
    }

    pub fn update(&mut self, buttons: Buttons) {
        let next = match self {
            Game::Subway(level) => level.update(buttons),
            Game::SeedScreen(level) => level.update(buttons),
        };
        if let Some(next) = next {
            *self = next;
        }
    }

    pub fn render(&mut self, fb: &mut Framebuffer) {
        match self {
            Game::Subway(level) => level.render(fb),
            Game::SeedScreen(level) => level.render(fb),
        }
    }

    pub fn render_ui<T, E>(&mut self, display: &mut T)
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        match self {
            Game::Subway(level) => level.render_ui(display),
            Game::SeedScreen(level) => level.render_ui(display),
        }
    }
}
