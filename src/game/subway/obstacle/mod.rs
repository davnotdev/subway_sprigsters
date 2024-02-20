use super::*;

const OBSTACLE_DESPAWN_Z: f32 = -2.0;
const OBSTACLE_BASE_STATIC_SPEED: f32 = 0.8;
pub const OBSTACLE_UNIT_BLOCK_LENGTH: f32 = 0.8;
const OBSTACLE_UNIT_SKINNY_LENGTH: f32 = 0.1;
const OBSTACLE_OVER_HEIGHT: f32 = 0.6;

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
        for (lane_idx, obstacles) in self.obstacles.iter_mut().enumerate() {
            let x_position = lane_idx as i8 - 1;

            for obstacle in obstacles.iter_mut() {
                obstacle.z_position -= OBSTACLE_BASE_STATIC_SPEED
            }

            obstacles.retain(|obstacle| obstacle.z_position >= OBSTACLE_DESPAWN_Z);

            for obstacle in obstacles.iter() {
                let len = obstacle.ty.get_length();

                if x_position == self.player.x_position
                    && (obstacle.z_position - len / 2.0 < 0.0
                        || obstacle.z_position + len / 2.0 < 0.0)
                {
                    if let PlayerMovement::Sliding { .. } = self.player.movement {
                        if let ObstacleType::Under = obstacle.ty {
                            continue;
                        }
                    }

                    if self.player.y_position > OBSTACLE_OVER_HEIGHT {
                        if let ObstacleType::Over = obstacle.ty {
                            continue;
                        }
                    }

                    self.player_died.signal();
                }
            }
        }
    }

    pub fn render_obstacles(&mut self, fb: &mut Framebuffer) {
        for (lane_idx, obstacles) in self.obstacles.iter().enumerate() {
            let x_position = lane_idx as i8 - 1;
            for obstacle in obstacles.iter() {
                let obstacle_length = obstacle.ty.get_length();
                let (triangles, scale, y_offset, color) = match obstacle.ty {
                    ObstacleType::Train { .. } => (
                        models::cube(),
                        [0.8, 2.0, obstacle_length],
                        0.0,
                        Color::Blue5,
                    ),
                    ObstacleType::RampTrain { .. } => (
                        models::cube(),
                        [0.8, 2.0, obstacle_length],
                        0.0,
                        Color::Blue7,
                    ),
                    ObstacleType::Over => (
                        models::quad(),
                        [0.6, OBSTACLE_OVER_HEIGHT, obstacle_length],
                        0.0,
                        Color::Pink8,
                    ),
                    ObstacleType::Under => (
                        models::quad(),
                        [0.6, 1.0, obstacle_length],
                        2.0,
                        Color::PurpleH,
                    ),
                };

                let model = mat4_identity();
                let model = mat4_scale(model, scale);
                let model = mat4_translate(
                    model,
                    [2.0 * x_position as f32, -y_offset, obstacle.z_position],
                );
                fb.render_pass(&RenderPass {
                    camera_front: consts::CAMERA_FRONT,
                    camera_position: consts::CAMERA_POSITION,
                    triangles,
                    model,
                    color: Some(color),
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
    }
}
