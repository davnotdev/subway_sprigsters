#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]
#![feature(type_alias_impl_trait)]

use arrayvec::{ArrayString, ArrayVec};
use embedded_graphics::{
    draw_target::DrawTarget,
    image::{Image, ImageRawLE},
    primitives::Rectangle,
    mono_font::{ascii, MonoTextStyle},
    pixelcolor::{Rgb565, Rgb888},
    prelude::*,
    text::Text,
};
use numtoa::NumToA;
use trowel::{App, AppResult, Buttons};

#[allow(unused_imports)]
use micromath::F32Ext;

mod color;
mod game;
mod graphics;
mod math;
mod models;
mod rand;
mod signal;

use color::Color;
use game::Game;
use graphics::*;
use math::*;
use rand::*;
use signal::*;

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
        self.game.render(&mut self.framebuffer, display);
        Ok(())
    }
}

#[trowel::entry]
fn main() {
    trowel::run(GameApp::new());
}
