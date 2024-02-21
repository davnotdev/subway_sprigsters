use super::*;

pub enum PlayerMovement {
    Running { last_frame_horizontal_input: bool },
    Sliding { roll_start_tick: u64 },
    Climbing,
}

pub struct Player {
    pub x_position: i8,
    pub y_position: f32,
    pub y_velocity: f32,
    pub movement: PlayerMovement,
    pub is_on_ground: bool,
    pub is_above_block: bool,
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
            is_on_ground: true,
            is_above_block: false,
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
                if self.ticks - *roll_start_tick > 9 {
                    self.player.movement = PlayerMovement::Running {
                        last_frame_horizontal_input: false,
                    }
                }
            }
            PlayerMovement::Climbing => {
                self.player.y_position += 0.2;
                if self.player.y_position >= OBSTACLE_CEIL_Y {
                    self.player.movement = PlayerMovement::Running {
                        last_frame_horizontal_input: false,
                    }
                }
            }
        }

        let is_player_climbing = matches!(self.player.movement, PlayerMovement::Climbing);
        if !is_player_climbing {
            self.player.y_velocity -= 0.4;
            self.player.y_position =
                (self.player.y_position + self.player.y_velocity).clamp(0.0, 999.0);
        }

        if self.player.is_on_ground {
            if self.player.y_position <= 0.1 {
                self.player.y_position = 0.0;
                self.can_jump = true;
            }
        } else if !is_player_climbing && self.player.y_position - OBSTACLE_CEIL_Y <= 0.1 && self.player.is_above_block {
            self.player.y_position = OBSTACLE_CEIL_Y + 0.05;
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
            PlayerMovement::Climbing => 1.0,
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
            camera_position: self.get_camera_position(),
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
