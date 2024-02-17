#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use embedded_graphics::{
    draw_target::DrawTarget,
    image::{Image, ImageRawLE},
    mono_font::{ascii, MonoTextStyle},
    pixelcolor::{Rgb565, Rgb888},
    prelude::*,
    text::Text,
};
use smallvec::{smallvec, SmallVec};
use std::time::{Duration, Instant};
use trowel::{App, AppResult, Buttons};

#[allow(unused_imports)]
use micromath::F32Ext;

mod color;
mod game;
mod graphics;
mod math;
mod models;
mod rand;

use color::Color;
use game::Game;
use graphics::*;
use math::*;
use rand::*;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 128;

struct GameApp {
    framebuffer: Framebuffer,
    game: Game,
}

impl GameApp {
    pub fn new() -> Self {
        Self {
            framebuffer: Framebuffer::new(),
            game: Game::new(),
        }
    }
}

impl App for GameApp {
    fn init(&mut self) -> AppResult {
        // self.game.init();
        Ok(())
    }

    fn update(&mut self, buttons: Buttons) -> AppResult {
        self.game.update(buttons);
        Ok(())
    }

    fn draw<T, E>(&mut self, display: &mut T) -> AppResult
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        // if self.game.ticks <= 2 {
        //     self.game.init_text(display);
        // }

        self.game.render(&mut self.framebuffer);
        self.framebuffer.flush(display);
        Ok(())
    }
}

#[trowel::entry]
fn main() {
    trowel::run(GameApp::new());
}
