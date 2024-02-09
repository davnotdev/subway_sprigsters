use super::*;

mod consts;
mod obsticle;
mod player;
mod spawner;

use obsticle::*;
use player::*;

pub struct SubwayLevel {
    pub ticks: u64,
    pub can_jump: bool,
    pub last_frame_horizontal_input: bool,
    pub player: Player,
    pub obsticles: SmallVec<[Obsticle; consts::MAX_OBSTICLES]>,
}

impl SubwayLevel {
    pub fn new() -> Self {
        Self {
            ticks: 0,
            can_jump: true,
            last_frame_horizontal_input: false,
            player: Player::new(),
            obsticles: smallvec![],
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

        self.render_obsticles(fb);
        self.render_player(fb);
    }
}
