use super::*;

const OBSTACLE_DESPAWN_Z: f32 = -10.0;
const OBSTACLE_BASE_STATIC_SPEED: f32 = 0.8;
pub const OBSTACLE_UNIT_BLOCK_LENGTH: f32 = 0.8;
const OBSTACLE_UNIT_SKINNY_LENGTH: f32 = 0.6;
const OBSTACLE_OVER_HEIGHT: f32 = 1.0;
const OBSTACLE_RAMP_LENGTH: f32 = 1.8;
pub const OBSTACLE_BLOCK_HEIGHT: f32 = 2.8;
pub const OBSTACLE_CEIL_Y: f32 = 3.0;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObstacleType {
    Train { count: u8 },
    RampTrain { count: u8 },
    Over,
    Under,
}

impl ObstacleType {
    pub fn new_rand(min_train: usize, max_train: usize) -> Self {
        let n = rand::rand_range::<u8, _>(0..=3);
        match n {
            0 => {
                let train_count = rand::rand_range::<_, _>(min_train..=max_train);
                Self::Train {
                    count: train_count as u8,
                }
            }
            1 => {
                let train_count = rand::rand_range::<_, _>(min_train..=max_train);
                Self::RampTrain {
                    count: train_count as u8,
                }
            }
            2 => Self::Over,
            3 => Self::Under,
            _ => panic!("Invalid u8 ObstacleType::from_u8"),
        }
    }

    pub fn get_length(&self) -> f32 {
        match self {
            ObstacleType::Train { count } => *count as f32 * OBSTACLE_UNIT_BLOCK_LENGTH,
            ObstacleType::RampTrain { count } => *count as f32 * OBSTACLE_UNIT_BLOCK_LENGTH,
            ObstacleType::Over => OBSTACLE_UNIT_SKINNY_LENGTH,
            ObstacleType::Under => OBSTACLE_UNIT_SKINNY_LENGTH,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Obstacle {
    pub ty: ObstacleType,
    pub z_position: f32,
}

impl SubwayLevel {
    pub fn update_obstacles(&mut self) {
        for obstacles in self.obstacles.iter_mut() {
            for obstacle in obstacles.iter_mut() {
                obstacle.z_position -= OBSTACLE_BASE_STATIC_SPEED * self.game_speed_scalar
            }

            obstacles.retain(|obstacle| obstacle.z_position >= OBSTACLE_DESPAWN_Z);
        }

        let mut is_above_block = false;
        if let Some(obstacles) = self.obstacles.get((self.player.x_position + 1) as usize) {
            for obstacle in obstacles.iter() {
                let len = obstacle.ty.get_length();

                if let ObstacleType::RampTrain { .. } = obstacle.ty {
                    if obstacle.z_position - len - OBSTACLE_RAMP_LENGTH * 2.0 < 0.0
                        && obstacle.z_position - len > 0.0
                        && self.player.y_position <= OBSTACLE_CEIL_Y
                    {
                        self.player.movement = PlayerMovement::Climbing;
                    }
                }

                if !matches!(self.player.movement, PlayerMovement::Climbing)
                    && obstacle.z_position - len < 0.0
                    && obstacle.z_position + len > 0.0
                {
                    if self.player.y_position < OBSTACLE_CEIL_Y {
                        let mut should_die = false;
                        if matches!(obstacle.ty, ObstacleType::Under)
                            && !matches!(self.player.movement, PlayerMovement::Sliding { .. })
                        {
                            should_die = true;
                        }

                        if self.player.y_position <= OBSTACLE_OVER_HEIGHT
                            && matches!(obstacle.ty, ObstacleType::Over)
                        {
                            should_die = true;
                        }

                        if matches!(
                            obstacle.ty,
                            ObstacleType::RampTrain { .. } | ObstacleType::Train { .. }
                        ) && self.player.y_position <= 0.2
                        {
                            should_die = true;
                        }

                        if should_die {
                            self.player_died.signal();
                        }
                    }

                    if matches!(obstacle.ty, ObstacleType::RampTrain { .. })
                        || matches!(obstacle.ty, ObstacleType::Train { .. })
                    {
                        is_above_block = true;
                    }
                }
            }
        }

        self.player.is_above_block = is_above_block;
    }

    pub fn render_obstacles(&mut self, fb: &mut Framebuffer) {
        for (lane_idx, obstacles) in self.obstacles.iter().enumerate() {
            let x_position = lane_idx as i8 - 1;
            for obstacle in obstacles.iter() {
                let obstacle_length = obstacle.ty.get_length();
                match obstacle.ty {
                    ObstacleType::Train { .. } => self.render_each(
                        fb,
                        x_position,
                        obstacle.z_position,
                        models::cube(),
                        [1.6, OBSTACLE_BLOCK_HEIGHT, obstacle_length],
                        0.0,
                        Color::Blue5,
                    ),
                    ObstacleType::RampTrain { .. } => {
                        self.render_each(
                            fb,
                            x_position,
                            obstacle.z_position,
                            models::cube(),
                            [1.6, OBSTACLE_BLOCK_HEIGHT, obstacle_length],
                            0.0,
                            Color::GreenD,
                        );
                        self.render_each(
                            fb,
                            x_position,
                            obstacle.z_position - obstacle_length - OBSTACLE_RAMP_LENGTH,
                            models::ramp(),
                            [1.6, OBSTACLE_BLOCK_HEIGHT, 2.0],
                            0.0,
                            Color::GreenD,
                        );
                    }
                    ObstacleType::Over => self.render_each(
                        fb,
                        x_position,
                        obstacle.z_position,
                        models::quad(),
                        [1.0, OBSTACLE_OVER_HEIGHT, obstacle_length],
                        0.0,
                        Color::Pink8,
                    ),
                    ObstacleType::Under => self.render_each(
                        fb,
                        x_position,
                        obstacle.z_position,
                        models::quad(),
                        [1.0, 1.0, obstacle_length],
                        2.0,
                        Color::PurpleH,
                    ),
                }
            }
        }
    }

    fn render_each(
        &self,
        fb: &mut Framebuffer,
        x_position: i8,
        z_position: f32,
        triangles: &[f32],
        scale: Vec3,
        y_offset: f32,
        color: Color,
    ) {
        let model = mat4_identity();
        let model = mat4_scale(model, scale);
        let model = mat4_translate(model, [4.0 * x_position as f32, -y_offset, z_position]);
        fb.render_pass(&RenderPass {
            camera_front: consts::CAMERA_FRONT,
            camera_position: self.get_camera_position(),
            triangles,
            model,
            color: Some(color),
            border_color: Some(Color::Gray2),
            enable_depth: true,
            projection: Some(ProjectionData {
                fov_rad: FOV_RAD,
                near: NEAR,
                far: FAR,
            }),
        })
    }
}
