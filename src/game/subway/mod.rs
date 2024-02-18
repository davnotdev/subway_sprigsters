use super::*;

mod consts;
mod ground;
mod obsticle;
mod player;
mod spawner;

use obsticle::*;
use player::*;

#[derive(Default, PartialEq, Eq)]
pub enum OnceSignal {
    #[default]
    Down,
    Up,
    Off,
}

impl OnceSignal {
    pub fn signal(&mut self) {
        if *self != OnceSignal::Off {
            *self = OnceSignal::Up
        }
    }

    pub fn try_take(&mut self) -> bool {
        if *self == OnceSignal::Up {
            *self = OnceSignal::Off;
            true
        } else {
            false
        }
    }

    pub fn is_off(&self) -> bool {
        *self == OnceSignal::Off
    }
}

pub struct SubwayLevel {
    pub ticks: u64,
    pub can_jump: bool,
    pub player: Player,
    pub obsticles: [ArrayVec<Obsticle, { consts::MAX_OBSTICLES }>; 3],

    pub player_died: OnceSignal,
}

impl SubwayLevel {
    pub fn new() -> Self {
        Self {
            ticks: 0,
            can_jump: true,
            player: Player::new(),
            obsticles: [ArrayVec::new(), ArrayVec::new(), ArrayVec::new()],

            player_died: Default::default(),
        }
    }

    pub fn update(&mut self, buttons: Buttons) {
        self.ticks += 1;
        self.update_player(buttons);
        self.update_obsticles();
        self.update_spawner();
    }

    pub fn render(&mut self, fb: &mut Framebuffer) {
        fb.clear_color(Color::GrayL);
        fb.clear_depth(core::f32::MAX);

        self.render_ground(fb);
        self.render_obsticles(fb);
        self.render_player(fb);
    }
}
