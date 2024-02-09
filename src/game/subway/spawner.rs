use super::*;

pub const OBSTICLE_Z_SPAWN: f32 = 15.0;

impl SubwayLevel {
    pub fn update_spawner(&mut self) {
        if self.ticks % 16 == 0 {
            let try_spawn = rand::<bool>();
            if try_spawn && check_spawn() {
                let x_position = rand_range::<i8, _>(0..=2);
                let ty = rand::rand_range::<u8, _>(0..=3);
                self.obsticles.push(Obsticle::new(
                    x_position,
                    OBSTICLE_Z_SPAWN,
                    ObsticleType::from_u8(ty),
                ));
            }
        }
    }
}

//  1. Ramps must be proceeded with blocks
//  2. Unders/Overs must not be followed by anything until
//      a. Ramps on same lane
//      b. An openning on any other lane
//  3. Moves prevent anything from spawning prior to and after spawning in
fn check_spawn() -> bool {
    true
}
