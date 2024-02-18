use super::*;

//  Spawn Rules
//  1. Ramps/Blocks must spawn X number of blocks behind them.
//  2. Unders/Overs much be padded w/ `OBSTICLE_JUMP_SLIDE_SAFETY_PADDING`.
//  3. Only spawn in two lanes at a time unless there is a ramp.

pub const OBSTICLE_Z_SPAWN: f32 = 45.0;
const OBSTICLE_JUMP_SLIDE_SAFETY_PADDING: f32 = 5.0;
const MAX_TRAIN_LENGTH: usize = 16;
const MIN_TRAIN_LENGTH: usize = 6;

impl SubwayLevel {
    pub fn update_spawner(&mut self) {
        if self.ticks % 32 == 0 {
            self.spawn();
        }
    }

    fn spawn(&mut self) {
        let tys = [
            ObsticleType::from_u8(rand::rand_range::<u8, _>(0..=3)),
            ObsticleType::from_u8(rand::rand_range::<u8, _>(0..=3)),
            ObsticleType::from_u8(rand::rand_range::<u8, _>(0..=3)),
        ];

        let spawn_one_more = if tys[0] == ObsticleType::Ramp
            || tys[1] == ObsticleType::Ramp
            || tys[2] == ObsticleType::Ramp
        {
            rand::<bool>()
        } else {
            false
        };

        let spawn_twice = rand::<bool>();
        let spawn_count = if spawn_twice { 2 } else { 1 } + if spawn_one_more { 1 } else { 0 };

        let mut lanes = [[None; MAX_TRAIN_LENGTH]; 3];

        for lane_i in 0..spawn_count {
            let ty = tys[lane_i];

            let z_offset = OBSTICLE_JUMP_SLIDE_SAFETY_PADDING;

            let obsticle = Obsticle {
                ty,
                z_position: OBSTICLE_Z_SPAWN + z_offset,
            };
            lanes[lane_i][0] = Some(obsticle);

            if ty == ObsticleType::Ramp || ty == ObsticleType::Block {
                let train_length = rand_range(MIN_TRAIN_LENGTH..MAX_TRAIN_LENGTH);
                for i in 1..train_length {
                    let obsticle = Obsticle {
                        ty: ObsticleType::Block,
                        z_position: OBSTICLE_Z_SPAWN
                            + z_offset
                            + i as f32 * (OBSTICLE_UNIT_BLOCK_LENGTH * 2.0 + 0.4),
                    };
                    lanes[lane_i][i] = Some(obsticle);
                }
            }
        }

        let should_rev = rand::<bool>();
        if should_rev {
            lanes.swap(0, 2);
        }

        for (lane, add_lane) in self.obsticles.iter_mut().zip(lanes.into_iter()) {
            for add_obsticle in add_lane.into_iter().flatten() {
                lane.push(add_obsticle);
            }
        }
    }
}
