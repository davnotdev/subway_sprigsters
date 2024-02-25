use super::*;

mod consts;
mod seed_screen;
mod start_screen;
mod subway;

use consts::*;
use seed_screen::*;
use start_screen::*;
use subway::*;

pub enum Game {
    Subway(SubwayLevel),
    SeedScreen(SeedScreen),
    StartScreen(StartScreen),
}

impl Game {
    pub fn new() -> Self {
        Self::SeedScreen(SeedScreen::new())
    }

    pub fn update(&mut self, buttons: Buttons) {
        let next = match self {
            Game::Subway(level) => level.update(buttons),
            Game::StartScreen(level) => level.update(buttons),
            Game::SeedScreen(level) => level.update(buttons),
        };
        if let Some(next) = next {
            *self = next;
        }
    }

    pub fn render<T, E>(&mut self, fb: &mut Framebuffer, display: &mut T)
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        match self {
            Game::Subway(level) => level.render(fb, display),
            Game::StartScreen(level) => level.render(fb, display),
            Game::SeedScreen(level) => level.render(fb, display),
        }
    }
}
