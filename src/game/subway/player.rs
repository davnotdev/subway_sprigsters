use super::*;

pub enum PlayerMovement {
    Running { last_frame_horizontal_input: bool },
    Sliding { roll_start_tick: u64 },
}

pub struct Player {
    pub x_position: i8,
    pub y_position: f32,
    pub y_velocity: f32,
    pub movement: PlayerMovement,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x_position: 0,
            y_position: 0.0,
            y_velocity: 0.0,
            movement: PlayerMovement::Running {
                last_frame_horizontal_input: false,
            },
        }
    }
}

impl SubwayLevel {
    pub fn update_player(&mut self, buttons: Buttons) {
        match &mut self.player.movement {
            PlayerMovement::Running {
                ref mut last_frame_horizontal_input,
            } => {
                if buttons.contains(Buttons::A) || buttons.contains(Buttons::J) {
                    if !*last_frame_horizontal_input {
                        self.player.x_position = (self.player.x_position - 1).clamp(-1, 1);
                    }
                    *last_frame_horizontal_input = true;
                } else if buttons.contains(Buttons::D) || buttons.contains(Buttons::L) {
                    if !*last_frame_horizontal_input {
                        self.player.x_position = (self.player.x_position + 1).clamp(-1, 1);
                    }
                    *last_frame_horizontal_input = true;
                } else {
                    *last_frame_horizontal_input = false;
                }

                if self.can_jump && buttons.contains(Buttons::I) {
                    self.player.y_velocity = 2.0;
                    self.can_jump = false;
                }

                if buttons.contains(Buttons::K) {
                    self.player.movement = PlayerMovement::Sliding {
                        roll_start_tick: self.ticks,
                    }
                }
            }
            PlayerMovement::Sliding {
                ref mut roll_start_tick,
            } => {
                if self.ticks - *roll_start_tick > 7 {
                    self.player.movement = PlayerMovement::Running {
                        last_frame_horizontal_input: false,
                    }
                }
            }
        }

        self.player.y_velocity -= 0.4;
        self.player.y_position = (self.player.y_position + self.player.y_velocity).clamp(0.0, 5.0);

        if self.player.y_position <= 0.2 {
            self.player.y_position = 0.0;
            self.can_jump = true;
        }

        if self.player_died.try_take() {}
    }

    pub fn render_player(&mut self, fb: &mut Framebuffer) {
        if self.player_died.is_off() {
            return;
        }

        let player_height_scalar = match self.player.movement {
            PlayerMovement::Running { .. } => 1.0 + (self.ticks as f32).sin() * 0.15,
            PlayerMovement::Sliding { .. } => 0.25,
        };

        let model = mat4_identity();
        let model = mat4_scale(model, [0.3, 0.6 * player_height_scalar, 0.3]);
        let model = mat4_rotate(model, 0.26, [1.0, 0.0, 0.0]);
        let model = mat4_translate(
            model,
            [
                2.0 * self.player.x_position as f32,
                -self.player.y_position,
                0.0,
            ],
        );

        fb.render_pass(&RenderPass {
            camera_front: consts::CAMERA_FRONT,
            camera_position: consts::CAMERA_POSITION,
            triangles: models::cube(),
            model,
            color: Some(Color::Gray2),
            border_color: Some(Color::Gray1),
            enable_depth: true,
            projection: Some(ProjectionData {
                fov_rad: FOV_RAD,
                near: NEAR,
                far: FAR,
            }),
        })
    }
}
