use super::*;

const OBSTICLE_DESPAWN_Z: f32 = -2.0;
const OBSTICLE_BASE_STATIC_SPEED: f32 = 0.8;

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
            let (triangles, scale) = match obsticle.ty {
                ObsticleType::Block => (models::cube(), [0.8, 2.0, 0.8]),
                ObsticleType::Ramp => (models::cube(), [0.8, 2.0, 0.8]),
                ObsticleType::Over => (models::cube(), [0.6, 0.6, 0.6]),
                ObsticleType::Under => (models::cube(), [0.6, 2.0, 0.1]),
            };

            let model = mat4_identity();
            let model = mat4_scale(model, scale);
            let model = mat4_translate(
                model,
                [2.0 * obsticle.x_position as f32, 0.0, obsticle.z_position],
            );
            fb.render_pass(&RenderPass {
                camera_front: vec_normalize([0.0, 0.1, 1.0]),
                camera_position: [0.0, -2.0, -8.0],
                triangles,
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
}
