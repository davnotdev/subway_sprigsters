//  Adapted from ROOM: https://github.com/davnotdev/Room
use super::*;

#[derive(Clone, Copy)]
pub struct ProjectionData {
    pub fov_rad: f32,
    pub near: f32,
    pub far: f32,
}

pub struct RenderPass<'triangles> {
    pub camera_front: Vec3,
    pub camera_position: Vec3,
    pub triangles: &'triangles [f32],
    pub model: Mat4,
    pub color: Option<Color>,
    pub border_color: Option<Color>,
    pub enable_depth: bool,
    pub projection: Option<ProjectionData>,
}

const FRAMEBUFFER_WIDTH: usize = 120;
const FRAMEBUFFER_HEIGHT: usize = 96;

pub struct Framebuffer {
    colors: [u16; FRAMEBUFFER_WIDTH * FRAMEBUFFER_HEIGHT],
    depths: [f32; FRAMEBUFFER_WIDTH * FRAMEBUFFER_HEIGHT],
}

impl Framebuffer {
    pub fn new() -> Self {
        Self {
            colors: [0; FRAMEBUFFER_WIDTH * FRAMEBUFFER_HEIGHT],
            depths: [0.0; FRAMEBUFFER_WIDTH * FRAMEBUFFER_HEIGHT],
        }
    }

    #[inline(always)]
    pub fn clear_color(&mut self, color: Color) {
        let val = Rgb565::from(color).into_storage();
        self.colors.iter_mut().for_each(|v| *v = val);
    }

    #[inline(always)]
    pub fn clear_depth(&mut self, value: f32) {
        self.depths.iter_mut().for_each(|v| *v = value)
    }

    #[inline(always)]
    fn put_pixel(&mut self, x: usize, y: usize, z: Option<f32>, color: Color) {
        if x < FRAMEBUFFER_WIDTH && y < FRAMEBUFFER_HEIGHT {
            let idx = FRAMEBUFFER_WIDTH * y + x;
            if let Some(z) = z {
                if z < self.depths[idx] {
                    self.depths[idx] = z;
                    self.colors[idx] = Rgb565::from(color).into_storage();
                }
            } else {
                self.colors[idx] = Rgb565::from(color).into_storage();
            }
        }
    }

    #[inline(always)]
    fn interpolate(i0: f32, d0: f32, i1: f32, d1: f32) -> ArrayVec<f32, 256> {
        if i0 == i1 {
            let mut ret = ArrayVec::new();
            ret.push(d0);
            return ret;
        }

        let mut values = ArrayVec::new();
        let a = (d1 - d0) / (i1 - i0);
        let mut d = d0;
        for _ in (i0 as usize)..(i1 as usize) {
            values.push(d);
            d += a;
        }

        values
    }

    #[inline(always)]
    pub fn draw_line(&mut self, mut a: Vec3, mut b: Vec3, color: Color, enable_depth: bool) {
        let dx = b[0] - a[0];
        let dy = b[1] - a[1];

        if dx.abs() > dy.abs() {
            if dx < 0.0 {
                core::mem::swap(&mut a, &mut b);
            }

            let ys = Self::interpolate(a[0], a[1], b[0], b[1]);
            for x in (a[0] as usize)..(b[0] as usize) {
                // Crude estimation of depth.
                self.put_pixel(
                    x,
                    ys[x - a[0] as usize] as usize,
                    enable_depth.then_some(a[2]),
                    color,
                );
            }
        } else {
            if dy < 0.0 {
                core::mem::swap(&mut a, &mut b);
            }

            let xs = Self::interpolate(a[1], a[0], b[1], b[0]);
            for y in (a[1] as usize)..(b[1] as usize) {
                // Crude estimation of depth.
                self.put_pixel(
                    xs[y - a[1] as usize] as usize,
                    y,
                    enable_depth.then_some(a[2]),
                    color,
                );
            }
        }
    }

    #[inline(always)]
    pub fn draw_triangle(&mut self, a: Vec3, b: Vec3, c: Vec3, color: Color, enable_depth: bool) {
        self.draw_line(a, b, color, enable_depth);
        self.draw_line(b, c, color, enable_depth);
        self.draw_line(a, c, color, enable_depth);
    }

    fn fill_triangle(
        &mut self,
        mut a: Vec3,
        mut b: Vec3,
        mut c: Vec3,
        color: Color,
        enable_depth: bool,
    ) {
        if b[1] < a[1] {
            core::mem::swap(&mut a, &mut b);
        }
        if c[1] < a[1] {
            core::mem::swap(&mut a, &mut c);
        }
        if c[1] < b[1] {
            core::mem::swap(&mut b, &mut c);
        }

        let mut x01 = Self::interpolate(a[1], a[0], b[1], b[0]);
        let mut x12 = Self::interpolate(b[1], b[0], c[1], c[0]);
        let mut x02 = Self::interpolate(a[1], a[0], c[1], c[0]);

        x01.pop();
        x12.pop();
        x02.pop();
        let mut x012 = x01.clone();
        for i in x12 {
            x012.push(i);
        }

        let m = x02.len() / 2;
        if m != 0 && m < x02.len() && m < x012.len() {
            let (x_left, x_right) = if !x02.is_empty() && !x012.is_empty() && x02[m] < x012[m] {
                (x02, x012)
            } else {
                (x012, x02)
            };

            for y in (a[1] as usize)..=(c[1] as usize) {
                let i = y - a[1] as usize;
                if i < x_left.len() && i < x_right.len() {
                    for x in (x_left[i] as usize)..=(x_right[i] as usize) {
                        // Crude estimation of depth.
                        self.put_pixel(x, y, enable_depth.then_some(a[2]), color);
                    }
                }
            }
        }
    }

    #[inline(always)]
    pub fn flush<T, E>(&mut self, display: &mut T)
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        let raw_image = ImageRawLE::<Rgb565>::new(
            unsafe {
                core::slice::from_raw_parts(
                    self.colors.as_ptr() as *const u8,
                    self.colors.len() * 2,
                )
            },
            FRAMEBUFFER_WIDTH as u32,
        );
        let image = Image::new(
            &raw_image,
            Point {
                x: (SCREEN_WIDTH / 2 - FRAMEBUFFER_WIDTH / 2) as i32,
                y: (SCREEN_HEIGHT / 2 - FRAMEBUFFER_HEIGHT / 2) as i32,
            },
        );
        let Ok(_) = image.draw(display) else {
            panic!("Failed to draw.");
        };
    }

    pub fn render_pass(&mut self, pass: &RenderPass) {
        pass.triangles.chunks(9).for_each(|triangles| {
            let mut vertices = triangles.chunks(3);
            let vertex_a: Vec3 = vertices.next().unwrap().try_into().unwrap();
            let vertex_b: Vec3 = vertices.next().unwrap().try_into().unwrap();
            let vertex_c: Vec3 = vertices.next().unwrap().try_into().unwrap();
            let world_vertex_a =
                vec4_into_vec3(mat4_mul_vec4(pass.model, vec3_into_vec4(vertex_a)));
            let world_vertex_b =
                vec4_into_vec3(mat4_mul_vec4(pass.model, vec3_into_vec4(vertex_b)));
            let world_vertex_c =
                vec4_into_vec3(mat4_mul_vec4(pass.model, vec3_into_vec4(vertex_c)));
            let normal = vec_normalize(vec3_cross_product(
                vec_sub_vec(world_vertex_b, world_vertex_a),
                vec_sub_vec(world_vertex_c, world_vertex_a),
            ));

            if vec_dot(normal, vec_sub_vec(world_vertex_a, pass.camera_position)) < 0.0 {
                let view = mat4_get_look_at(
                    pass.camera_position,
                    vec_add_vec(pass.camera_position, pass.camera_front),
                    [0.0, 1.0, 0.0],
                );
                let view_vertex_a =
                    vec4_into_vec3(mat4_mul_vec4(view, vec3_into_vec4(world_vertex_a)));
                let view_vertex_b =
                    vec4_into_vec3(mat4_mul_vec4(view, vec3_into_vec4(world_vertex_b)));
                let view_vertex_c =
                    vec4_into_vec3(mat4_mul_vec4(view, vec3_into_vec4(world_vertex_c)));

                let clipped_triangles = if let Some(projection) = &pass.projection {
                    let test_planes = [
                        ([0.0, 0.0, projection.near], [0.0, 0.0, 1.0]),
                        ([0.0, 0.0, projection.far], [0.0, 0.0, -1.0]),
                    ];
                    Self::clip_planes((view_vertex_a, view_vertex_b, view_vertex_c), &test_planes)
                } else {
                    let mut ret = ArrayVec::new();
                    ret.push((view_vertex_a, view_vertex_b, view_vertex_c));
                    ret
                };

                clipped_triangles.iter().for_each(|triangle| {
                    let (vertex_a, vertex_b, vertex_c) = if let Some(projection) = &pass.projection
                    {
                        let projection = mat4_get_projection(
                            1.0,
                            projection.fov_rad,
                            projection.near,
                            projection.far,
                        );
                        let projected_vertex_a =
                            mat4_mul_vec4(projection, vec3_into_vec4(triangle.0));
                        let projected_vertex_b =
                            mat4_mul_vec4(projection, vec3_into_vec4(triangle.1));
                        let projected_vertex_c =
                            mat4_mul_vec4(projection, vec3_into_vec4(triangle.2));

                        (
                            vec4_into_vec3(vec4_scale_with_w(projected_vertex_a)),
                            vec4_into_vec3(vec4_scale_with_w(projected_vertex_b)),
                            vec4_into_vec3(vec4_scale_with_w(projected_vertex_c)),
                        )
                    } else {
                        (vertex_a, vertex_b, vertex_c)
                    };

                    let mut vertex_a = vec_add_scalar(vertex_a, 1.0);
                    let mut vertex_b = vec_add_scalar(vertex_b, 1.0);
                    let mut vertex_c = vec_add_scalar(vertex_c, 1.0);
                    vertex_a[0] *= (FRAMEBUFFER_WIDTH / 2) as f32;
                    vertex_b[0] *= (FRAMEBUFFER_WIDTH / 2) as f32;
                    vertex_c[0] *= (FRAMEBUFFER_WIDTH / 2) as f32;

                    vertex_a[1] *= (FRAMEBUFFER_HEIGHT / 2) as f32;
                    vertex_b[1] *= (FRAMEBUFFER_HEIGHT / 2) as f32;
                    vertex_c[1] *= (FRAMEBUFFER_HEIGHT / 2) as f32;

                    vertex_a[2] *= 100.0;
                    vertex_b[2] *= 100.0;
                    vertex_c[2] *= 100.0;

                    let test_planes = [
                        ([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
                        (
                            [0.0, FRAMEBUFFER_HEIGHT as f32 - 1.0, 0.0],
                            [0.0, -1.0, 0.0],
                        ),
                        ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
                        ([FRAMEBUFFER_WIDTH as f32 - 1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]),
                    ];

                    let final_triangles =
                        Self::clip_planes((vertex_a, vertex_b, vertex_c), &test_planes);

                    final_triangles.iter().for_each(|&(a, b, c)| {
                        if let Some(border_color) = pass.border_color {
                            self.draw_triangle(a, b, c, border_color, pass.enable_depth);
                        }
                        if let Some(color) = pass.color {
                            self.fill_triangle(a, b, c, color, pass.enable_depth);
                        }
                    });
                });
            }
        });
    }

    #[inline(always)]
    fn clip_planes(
        vertices: (Vec3, Vec3, Vec3),
        test_planes: &[(Vec3, Vec3)],
    ) -> ArrayVec<(Vec3, Vec3, Vec3), 4> {
        let mut final_triangles: ArrayVec<(Vec3, Vec3, Vec3), 4> = ArrayVec::new();
        final_triangles.push(vertices);

        for plane in test_planes {
            let mut passed: ArrayVec<(Vec3, Vec3, Vec3), 4> = ArrayVec::new();
            for t in final_triangles {
                for i in triangle_clip_plane(plane.0, plane.1, t) {
                    passed.push(i);
                }
            }
            final_triangles = passed;
        }
        final_triangles
    }
}
