use super::*;

pub const OBSTICLE_Z_SPAWN: f32 = 45.0;

impl SubwayLevel {
    pub fn update_spawner(&mut self) {
        if self.ticks % 8 == 0 {
            let try_spawn = rand::<bool>();
            if try_spawn {
                let x_position = rand_range::<i8, _>(0..=2);
                let ty = ObsticleType::from_u8(rand::rand_range::<u8, _>(0..=3));
                if check_spawn(&ty, x_position) {
                    self.obsticles
                        .push(Obsticle::new(x_position, OBSTICLE_Z_SPAWN, ty));
                }
            }
        }
    }
}

//  1. Ramps must be proceeded with blocks
//  2. Unders/Overs must not be followed by anything until
//      a. Ramps on same lane
//      b. An opening on any other lane
//  3. Moves prevent anything from spawning prior to and after spawning in
fn check_spawn(ty: &ObsticleType, x_position: i8) -> bool {
    true
}
