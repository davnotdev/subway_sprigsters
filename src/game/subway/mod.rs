use super::*;

mod obsticle;
mod player;

use obsticle::*;
use player::*;

pub struct SubwayLevel {
    pub ticks: u64,
    pub can_jump: bool,
    pub last_frame_horizontal_input: bool,
    pub player: Player,
}

impl SubwayLevel {
    pub fn new() -> Self {
        Self {
            ticks: 0,
            can_jump: true,
            last_frame_horizontal_input: false,
            player: Player::new(),
        }
    }

    pub fn update(&mut self, buttons: Buttons) {
        self.ticks += 1;
        self.update_player(buttons);
    }

    pub fn render(&mut self, fb: &mut Framebuffer) {
        fb.clear_color(Color::GrayL);
        fb.clear_depth(core::f32::MAX);

        self.render_player(fb);
    }
}
