use super::*;

const ANIMATION_SPEED: f32 = 0.6;
const ANIMATION_SUBWAY_SPEED: f32 = 1.0;

pub struct StartScreen {
    ticks: u64,
    initial_ui: bool,

    camera_front: Vec3,
    camera_position: Vec3,

    obstacle_z: f32,
    obstacle_color: Color,
    obstacle_length: u8,

    begin_start_animation: bool,
}

impl StartScreen {
    pub fn new() -> Self {
        Self {
            ticks: 0,
            initial_ui: true,
            camera_front: [-1.0, 0.0, 0.0],
            camera_position: [10.0, -1.0, 0.0],
            obstacle_z: 20.0,
            obstacle_color: Color::GreenD,
            obstacle_length: 4,
            begin_start_animation: false,
        }
    }

    pub fn update(&mut self, buttons: Buttons) -> Option<Game> {
        self.ticks += 1;

        self.obstacle_z -= ANIMATION_SUBWAY_SPEED;
        if self.obstacle_z <= -40.0 && rand::rand_range::<u8, _>(0..20) == 0 {
            self.obstacle_z = 40.0;
            self.obstacle_color = if rand::<bool>() {
                Color::GreenD
            } else {
                Color::Blue5
            };
            self.obstacle_length = rand_range::<u8, _>(4..=10);
        }

        if buttons.contains(Buttons::S) {
            self.begin_start_animation = true;
        }

        if self.begin_start_animation {
            let next_pos = vec_add_vec(
                self.camera_position,
                vec_mul_scalar(
                    vec_normalize(vec_sub_vec(
                        subway::consts::CAMERA_POSITION,
                        self.camera_position,
                    )),
                    ANIMATION_SPEED,
                ),
            );
            self.camera_position = next_pos;

            let look_at = vec_mul_scalar(subway::consts::CAMERA_FRONT, 4.0);
            self.camera_front = vec_normalize(vec_sub_vec(look_at, next_pos));

            if (self.camera_position[1] - subway::consts::CAMERA_POSITION[1]) < 0.001 {
                return Some(Game::Subway(SubwayLevel::new()));
            }
        }

        None
    }

    pub fn render<T, E>(&mut self, fb: &mut Framebuffer, display: &mut T)
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        if self.initial_ui {
            let Ok(_) = display.clear(Rgb565::BLACK) else {
                panic!("Failed to draw");
            };

            let style = MonoTextStyle::new(&ascii::FONT_8X13, Rgb565::WHITE);
            let Ok(_) = Text::new("Subway Sprigsters", Point::new(10, 16), style).draw(display)
            else {
                panic!("Failed to draw.");
            };
            let style = MonoTextStyle::new(&ascii::FONT_6X10, Rgb565::WHITE);
            let Ok(_) = Text::new("Press to S", Point::new(11, 34), style).draw(display) else {
                panic!("Failed to draw.");
            };

            self.initial_ui = false;
        }

        fb.clear_color(Color::Gray1);
        fb.clear_depth(core::f32::MAX);

        let model = mat4_identity();
        let model = mat4_scale(model, [8.0, 23.0, 0.0]);
        let model = mat4_rotate(model, core::f32::consts::FRAC_PI_2, [1.0, 0.0, 0.0]);
        let model = mat4_translate(model, [0.0, 0.0, 0.0]);

        fb.render_pass(&RenderPass {
            camera_front: self.camera_front,
            camera_position: self.camera_position,
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
        });

        if !self.begin_start_animation {
            let model = mat4_identity();
            let model = mat4_scale(
                model,
                [
                    1.6,
                    subway::OBSTACLE_BLOCK_HEIGHT,
                    self.obstacle_length as f32,
                ],
            );
            let model = mat4_translate(model, [-8.0, -1.0, self.obstacle_z]);
            fb.render_pass(&RenderPass {
                camera_front: self.camera_front,
                camera_position: self.camera_position,
                triangles: models::cube(),
                model,
                color: Some(self.obstacle_color),
                border_color: Some(Color::Gray2),
                enable_depth: true,
                projection: Some(ProjectionData {
                    fov_rad: FOV_RAD,
                    near: NEAR,
                    far: FAR,
                }),
            });
        }

        let player_scale_factor = (self.ticks as f32 / 10.0).cos().sin() * 0.1;
        let model = mat4_identity();
        let model = mat4_scale(model, [0.4, 0.75 + player_scale_factor, 0.4]);
        let model = mat4_translate(model, [0.0, -player_scale_factor, 0.0]);

        fb.render_pass(&RenderPass {
            camera_front: self.camera_front,
            camera_position: self.camera_position,
            triangles: models::cube(),
            model,
            color: Some(Color::Gray2),
            border_color: Some(Color::Gray0),
            enable_depth: true,
            projection: Some(ProjectionData {
                fov_rad: FOV_RAD,
                near: NEAR,
                far: FAR,
            }),
        });

        fb.flush(display);
    }
}
