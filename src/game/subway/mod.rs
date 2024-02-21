use super::*;

mod consts;
mod ground;
mod obstacle;
mod player;
mod spawner;

use obstacle::*;
use player::*;

pub struct SubwayLevel {
    pub ticks: u64,
    pub can_jump: bool,
    pub player: Player,
    pub obstacles: [ArrayVec<Obstacle, { consts::MAX_OBSTACLES }>; 3],

    pub initial_ui: bool,
    pub player_died: OnceSignal,
}

impl SubwayLevel {
    pub fn new() -> Self {
        Self {
            ticks: 0,
            can_jump: true,
            player: Player::new(),
            obstacles: [ArrayVec::new(), ArrayVec::new(), ArrayVec::new()],

            initial_ui: true,
            player_died: Default::default(),
        }
    }

    pub fn update(&mut self, buttons: Buttons) -> Option<Game> {
        self.ticks += 1;

        self.update_player(buttons);
        self.update_obstacles();
        self.update_spawner();

        None
    }

    pub fn render<T, E>(&mut self, fb: &mut Framebuffer, display: &mut T)
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        if self.initial_ui {
            let Ok(_) = display.clear(Rgb565::BLACK) else {
                panic!("Failed to draw");
            };
            self.initial_ui = false;
        }

        fb.clear_color(Color::GrayL);
        fb.clear_depth(core::f32::MAX);

        self.render_ground(fb);
        self.render_obstacles(fb);
        self.render_player(fb);

        fb.flush(display);
    }

    pub fn get_camera_position(&self) -> Vec3 {
        vec_sub_vec(consts::CAMERA_POSITION, [0.0, self.player.y_position, 0.0])
    }
}
