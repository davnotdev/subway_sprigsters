use super::*;

mod player;

use player::*;

const FOV_RAD: f32 = core::f32::consts::FRAC_PI_4;
const NEAR: f32 = 0.1;
const FAR: f32 = 50.0;

pub struct Game {
    ticks: u64,
    can_jump: bool,
    last_frame_horizontal_input: bool,
    player: Player,
}

impl Game {
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
