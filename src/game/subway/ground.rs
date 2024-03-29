use super::*;

impl SubwayLevel {
    pub fn render_ground(&self, fb: &mut Framebuffer) {
        let model = mat4_identity();
        let model = mat4_scale(model, [8.0, 23.0, 0.0]);
        let model = mat4_rotate(model, core::f32::consts::FRAC_PI_2, [1.0, 0.0, 0.0]);
        let model = mat4_translate(model, [0.0, 1.0, 20.0]);

        fb.render_pass(&RenderPass {
            camera_front: consts::CAMERA_FRONT,
            camera_position: self.get_camera_position(),
            triangles: models::quad(),
            model,
            color: Some(Color::GrayL),
            border_color: Some(Color::GrayL),
            enable_depth: false,
            projection: Some(ProjectionData {
                fov_rad: FOV_RAD,
                near: NEAR,
                far: FAR,
            }),
        })
    }
}
