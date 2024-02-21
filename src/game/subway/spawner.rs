use super::*;

//  Spawn Rules
//  1. Ramps/Blocks must spawn X number of blocks behind them.
//  2. Unders/Overs much be padded w/ `OBSTACLE_JUMP_SLIDE_SAFETY_PADDING`.
//  3. Only spawn in two lanes at a time unless there is a ramp.

pub const OBSTACLE_Z_SPAWN: f32 = 45.0;
const OBSTACLE_JUMP_SLIDE_SAFETY_PADDING: f32 = 5.0;
const MAX_TRAIN_LENGTH: usize = 16;
const MIN_TRAIN_LENGTH: usize = 6;

impl SubwayLevel {
    pub fn update_spawner(&mut self) {
        if self.ticks % 25 == 0 {
            self.spawn();
        }
    }

    fn spawn(&mut self) {
        let tys = [
            ObstacleType::new_rand(MIN_TRAIN_LENGTH, MAX_TRAIN_LENGTH),
            ObstacleType::new_rand(MIN_TRAIN_LENGTH, MAX_TRAIN_LENGTH),
            ObstacleType::new_rand(MIN_TRAIN_LENGTH, MAX_TRAIN_LENGTH),
        ];

        let spawn_one_more = if tys.iter().any(|o| {
            core::mem::discriminant(o)
                == core::mem::discriminant(&ObstacleType::RampTrain { count: 0 })
        }) {
            rand::<bool>()
        } else {
            false
        };

        let spawn_twice = rand::<bool>();
        let spawn_count = if spawn_twice { 2 } else { 1 } + if spawn_one_more { 1 } else { 0 };

        let mut lanes = [[None; MAX_TRAIN_LENGTH]; 3];
        let z_offset = OBSTACLE_JUMP_SLIDE_SAFETY_PADDING;

        for lane_i in 0..spawn_count {
            let ty = tys[lane_i];

            let obstacle = Obstacle {
                ty,
                z_position: OBSTACLE_Z_SPAWN + z_offset,
            };
            lanes[lane_i][0] = Some(obstacle);
        }

        let should_rev = rand::<bool>();
        if should_rev {
            lanes.swap(0, 2);
        }

        for (lane, add_lane) in self.obstacles.iter_mut().zip(lanes.into_iter()) {
            for add_obstacle in add_lane.into_iter().flatten() {
                lane.push(add_obstacle);
            }
        }
    }
}
