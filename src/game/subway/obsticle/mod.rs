use super::*;

const OBSTICLE_DESPAWN_Z: f32 = -2.0;
const OBSTICLE_BASE_STATIC_SPEED: f32 = 0.8;
const OBSTICLE_UNIT_BLOCK_LENGTH: f32 = 0.8;
const OBSTICLE_UNIT_SKINNY_LENGTH: f32 = 0.1;
const OBSTICLE_OVER_HEIGHT: f32 = 0.6;

#[repr(u8)]
pub enum ObsticleType {
    Block,
    Ramp,
    Over,
    Under,
}

impl ObsticleType {
    pub fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::Block,
            1 => Self::Ramp,
            2 => Self::Over,
            3 => Self::Under,
            _ => panic!("Invalid u8 ObsticleType::from_u8"),
        }
    }

    pub fn get_unit_length(&self) -> f32 {
        match self {
            ObsticleType::Block => OBSTICLE_UNIT_BLOCK_LENGTH,
            ObsticleType::Ramp => OBSTICLE_UNIT_BLOCK_LENGTH,
            ObsticleType::Over => OBSTICLE_UNIT_SKINNY_LENGTH,
            ObsticleType::Under => OBSTICLE_UNIT_SKINNY_LENGTH,
        }
    }
}

pub struct Obsticle {
    ty: ObsticleType,
    z_position: f32,
    x_position: i8,
}

impl Obsticle {
    pub fn new(x_position: i8, z_position: f32, ty: ObsticleType) -> Self {
        Self {
            ty,
            z_position,
            x_position,
        }
    }
}

impl SubwayLevel {
    pub fn update_obsticles(&mut self) {
        for obsticle in self.obsticles.iter_mut() {
            obsticle.z_position -= OBSTICLE_BASE_STATIC_SPEED
        }

        self.obsticles
            .retain(|obsticle| obsticle.z_position >= OBSTICLE_DESPAWN_Z);
    }

    pub fn render_obsticles(&mut self, fb: &mut Framebuffer) {
        for obsticle in self.obsticles.iter() {
            let obsticle_unit_length = obsticle.ty.get_unit_length();
            let (triangles, scale, y_offset, color) = match obsticle.ty {
                ObsticleType::Block => (
                    models::cube(),
                    [0.8, 2.0, obsticle_unit_length],
                    0.0,
                    Color::Blue5,
                ),
                ObsticleType::Ramp => (
                    models::cube(),
                    [0.8, 2.0, obsticle_unit_length],
                    0.0,
                    Color::Blue7,
                ),
                ObsticleType::Over => (
                    models::quad(),
                    [0.6, OBSTICLE_OVER_HEIGHT, obsticle_unit_length],
                    0.0,
                    Color::Pink8,
                ),
                ObsticleType::Under => (
                    models::quad(),
                    [0.6, 1.0, obsticle_unit_length],
                    2.0,
                    Color::PurpleH,
                ),
            };

            let model = mat4_identity();
            let model = mat4_scale(model, scale);
            let model = mat4_translate(
                model,
                [
                    2.0 * obsticle.x_position as f32,
                    -y_offset,
                    obsticle.z_position,
                ],
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

    pub fn update_collisions(&mut self) {
        for obsticle in self.obsticles.iter() {
            let len = obsticle.ty.get_unit_length();

            if obsticle.x_position == self.player.x_position
                && (obsticle.z_position - len / 2.0 < 0.0 || obsticle.z_position + len / 2.0 < 0.0)
            {
                if let PlayerMovement::Sliding { .. } = self.player.movement {
                    if let ObsticleType::Under = obsticle.ty {
                        continue;
                    }
                }

                if self.player.y_position > OBSTICLE_OVER_HEIGHT {
                    if let ObsticleType::Over = obsticle.ty {
                        continue;
                    }
                }

                self.player_died.signal();
            }
        }
    }
}
