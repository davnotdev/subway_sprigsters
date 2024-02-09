use super::*;

pub struct Obsticle {
    z_position: i8,
    x_position: i8,
}

impl Obsticle {
    pub fn new(x_position: i8, z_position: i8) -> Self {
        Self {
            z_position,
            x_position,
        }
    }
}
