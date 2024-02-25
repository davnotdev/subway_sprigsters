use super::*;

pub mod consts;
mod ground;
mod obstacle;
mod player;
mod spawner;
mod ui;

pub use obstacle::*;
pub use player::*;

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
        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        {
            fn expensive_computation(i: u32) {
                let mut fb = Framebuffer::new();
                let model = mat4_identity();
                let model = mat4_translate(model, [0.0, 0.0, 0.0]);
                core::hint::black_box(fb.render_pass(&RenderPass {
                    camera_front: [2.0, 0.0, 0.0],
                    camera_position: [i as f32 * 0.01, 0.0, 0.0],
                    triangles: models::cube(),
                    model,
                    color: Some(Color::Gray0),
                    border_color: Some(Color::Gray0),
                    enable_depth: true,
                    projection: Some(ProjectionData {
                        fov_rad: FOV_RAD,
                        near: NEAR,
                        far: FAR,
                    }),
                }));
            }
            for i in 0..200 {
                core::hint::black_box(expensive_computation(i));
            }
        }

        if self.ticks % 20 == 0 && !self.player_died.is_off() {
            self.game_speed_scalar += 0.0007;
        }

        if !self.player_died.is_off() {
            self.update_player(buttons);
        } else if buttons.contains(Buttons::K) {
            return Some(Game::StartScreen(StartScreen::new()));
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
