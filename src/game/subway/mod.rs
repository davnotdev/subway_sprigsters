use super::*;

mod consts;
mod ground;
mod obstacle;
mod player;
mod spawner;
mod ui;

use obstacle::*;
use player::*;
use ui::*;

pub struct SubwayLevel {
    pub ticks: u64,
    pub can_jump: bool,
    pub player: Player,
    pub obstacles: [ArrayVec<Obstacle, { consts::MAX_OBSTACLES }>; 3],

    pub initial_ui: bool,
    pub player_died: OnceSignal,

    pub game_speed_scalar: f32,
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
            game_speed_scalar: consts::STARTER_GAME_SPEED_SCALAR,
        }
    }

    pub fn update(&mut self, buttons: Buttons) -> Option<Game> {
        self.ticks += 1;

        if self.ticks % 20 == 0 && !self.player_died.is_off() {
            self.game_speed_scalar += 0.0007;
        }

        if !self.player_died.is_off() {
            self.update_player(buttons);
        }
        self.update_obstacles();
        self.update_spawner();

        None
    }

    pub fn render<T, E>(&mut self, fb: &mut Framebuffer, display: &mut T)
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        if self.ticks % 2 == 0 {
            self.render_ui(display);

            fb.clear_color(Color::Gray1);
            fb.clear_depth(core::f32::MAX);

            self.render_ground(fb);
            self.render_obstacles(fb);
            if !self.player_died.is_off() {
                self.render_player(fb);
            }

            fb.flush(display);
        }
    }

    pub fn get_camera_position(&self) -> Vec3 {
        vec_sub_vec(consts::CAMERA_POSITION, [0.0, self.player.y_position, 0.0])
    }
}
