use super::*;

pub struct Player {
    x_position: i8,
    y_position: f32,
    y_velocity: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x_position: 0,
            y_position: 0.0,
            y_velocity: 0.0,
        }
    }
}

impl SubwayLevel {
    pub fn update_player(&mut self, buttons: Buttons) {
        if buttons.contains(Buttons::A) || buttons.contains(Buttons::J) {
            if !self.last_frame_horizontal_input {
                self.player.x_position = (self.player.x_position - 1).clamp(-1, 1);
            }
            self.last_frame_horizontal_input = true;
        } else if buttons.contains(Buttons::D) || buttons.contains(Buttons::L) {
            if !self.last_frame_horizontal_input {
                self.player.x_position = (self.player.x_position + 1).clamp(-1, 1);
            }
            self.last_frame_horizontal_input = true;
        } else {
            self.last_frame_horizontal_input = false;
        }

        if self.can_jump && buttons.contains(Buttons::I) {
            self.player.y_velocity = 2.0;
            self.can_jump = false;
        }

        self.player.y_velocity -= 0.4;
        self.player.y_position = (self.player.y_position + self.player.y_velocity).clamp(0.0, 5.0);

        if self.player.y_position <= 0.2 {
            self.player.y_position = 0.0;
            self.can_jump = true;
        }
    }

    pub fn render_player(&mut self, fb: &mut Framebuffer) {
        let model = mat4_identity();
        let model = mat4_scale(model, [0.3, 0.6, 0.3]);
        let model = mat4_translate(
            model,
            [
                2.0 * self.player.x_position as f32,
                -self.player.y_position,
                0.0,
            ],
        );

        fb.render_pass(&RenderPass {
            camera_front: vec_normalize([0.0, 0.1, 1.0]),
            camera_position: [0.0, -2.0, -8.0],
            triangles: models::cube(),
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
