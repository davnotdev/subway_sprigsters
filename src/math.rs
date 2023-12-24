//  Adapted from ROOM: https://github.com/davnotdev/Room
use super::*;

pub type Vec<const V: usize> = [f32; V];
pub type Vec3 = Vec<3>;
pub type Vec4 = Vec<4>;
pub type Mat4 = [Vec4; 4];

#[inline(always)]
pub fn mat4_identity() -> Mat4 {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

#[inline(always)]
pub fn mat4_translate(mut mat: Mat4, translation: Vec3) -> Mat4 {
    mat[3][0] += translation[0];
    mat[3][1] += translation[1];
    mat[3][2] += translation[2];
    mat
}

#[inline(always)]
pub fn mat4_scale(mut mat: Mat4, scale: Vec3) -> Mat4 {
    mat[0][0] *= scale[0];
    mat[1][1] *= scale[1];
    mat[2][2] *= scale[2];
    mat
}

#[inline(always)]
pub fn mat4_rotate(mat: Mat4, angle: f32, rot: Vec3) -> Mat4 {
    let c = angle.cos();
    let s = angle.sin();
    let axis = vec_normalize(rot);
    let rot_mat = [
        [
            c + axis[0] * axis[0] * (1.0 - c),
            axis[0] * axis[1] * (1.0 - c) - axis[2] * s,
            axis[0] * axis[2] * (1.0 - c) + axis[1] * s,
            0.0,
        ],
        [
            axis[1] * axis[0] * (1.0 - c) + axis[2] * s,
            c + axis[1] * axis[1] * (1.0 - c),
            axis[1] * axis[2] * (1.0 - c) - axis[0] * s,
            0.0,
        ],
        [
            axis[2] * axis[0] * (1.0 - c) - axis[1] * s,
            axis[2] * axis[1] * (1.0 - c) + axis[0] * s,
            c + axis[2] * axis[2] * (1.0 - c),
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ] as Mat4;

    mat4_mul_mat4(mat, rot_mat)
}

#[inline(always)]
pub fn mat4_get_projection(aspect: f32, fov_rad: f32, near: f32, far: f32) -> Mat4 {
    [
        [aspect / (fov_rad / 2.0).tan(), 0.0, 0.0, 0.0],
        [0.0, 1.0 / (fov_rad / 2.0).tan(), 0.0, 0.0],
        [0.0, 0.0, far / (far - near), 1.0],
        [0.0, 0.0, (-far * near) / (far - near), 0.0],
    ]
}

#[inline(always)]
pub fn mat4_get_look_at(position: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    let dir = vec_sub_vec(center, position);
    let right = vec_normalize(vec3_cross_product(up, dir));
    let mat_up = vec3_cross_product(dir, right);

    [
        [right[0], mat_up[0], dir[0], 0.0],
        [right[1], mat_up[1], dir[1], 0.0],
        [right[2], mat_up[2], dir[2], 0.0],
        [
            -vec_dot(position, right),
            -vec_dot(position, mat_up),
            -vec_dot(position, dir),
            1.0,
        ],
    ]
}

#[inline(always)]
pub fn mat4_mul_mat4(b: Mat4, a: Mat4) -> Mat4 {
    let mut result = mat4_identity();
    for r in 0..4 {
        for c in 0..4 {
            result[r][c] =
                b[r][0] * a[0][c] + b[r][1] * a[1][c] + b[r][2] * a[2][c] + b[r][3] * a[3][c];
        }
    }
    result
}

#[inline(always)]
pub fn mat4_mul_vec4(mat: Mat4, vec: Vec4) -> Vec4 {
    let mut result = [0.0, 0.0, 0.0, 0.0] as Vec4;

    result[0] = vec[0] * mat[0][0] + vec[1] * mat[1][0] + vec[2] * mat[2][0] + vec[3] * mat[3][0];
    result[1] = vec[0] * mat[0][1] + vec[1] * mat[1][1] + vec[2] * mat[2][1] + vec[3] * mat[3][1];
    result[2] = vec[0] * mat[0][2] + vec[1] * mat[1][2] + vec[2] * mat[2][2] + vec[3] * mat[3][2];
    result[3] = vec[0] * mat[0][3] + vec[1] * mat[1][3] + vec[2] * mat[2][3] + vec[3] * mat[3][3];

    result
}

#[inline(always)]
pub fn vec4_scale_with_w(mut vec: Vec4) -> Vec4 {
    if vec[3] != 0.0 {
        vec[0] /= vec[3];
        vec[1] /= vec[3];
        vec[2] /= vec[3];
    }
    vec
}

#[inline(always)]
pub fn vec3_into_vec4(vec: Vec3) -> Vec4 {
    [vec[0], vec[1], vec[2], 1f32]
}

#[inline(always)]
pub fn vec4_into_vec3(vec: Vec4) -> Vec3 {
    [vec[0], vec[1], vec[2]]
}

#[inline(always)]
pub fn vec3_cross_product(a: Vec3, b: Vec3) -> Vec3 {
    let mut result = [0.0, 0.0, 0.0] as Vec3;
    result[0] = a[1] * b[2] - a[2] * b[1];
    result[1] = a[2] * b[0] - a[0] * b[2];
    result[2] = a[0] * b[1] - a[1] * b[0];
    result
}

#[inline(always)]
pub fn vec_add_vec<const V: usize>(mut a: Vec<V>, b: Vec<V>) -> Vec<V> {
    for (a, b) in a.iter_mut().zip(b.iter()) {
        *a += b;
    }
    a
}

#[inline(always)]
pub fn vec_sub_vec<const V: usize>(mut a: Vec<V>, b: Vec<V>) -> Vec<V> {
    for (a, b) in a.iter_mut().zip(b.iter()) {
        *a -= b;
    }
    a
}

#[inline(always)]
pub fn vec_distance<const V: usize>(a: Vec<V>, b: Vec<V>) -> f32 {
    vec_length(vec_sub_vec(b, a))
}

#[inline(always)]
pub fn vec_add_scalar<const V: usize>(mut v: [f32; V], s: f32) -> [f32; V] {
    for vs in v.iter_mut() {
        *vs += s;
    }
    v
}

#[inline(always)]
pub fn vec_mul_scalar<const V: usize>(mut v: [f32; V], s: f32) -> [f32; V] {
    for vs in v.iter_mut() {
        *vs *= s;
    }
    v
}

#[inline(always)]
pub fn vec_normalize<const V: usize>(v: [f32; V]) -> [f32; V] {
    let mut squared_sum = 0.0;
    for vs in v {
        squared_sum += vs * vs;
    }
    vec_mul_scalar(v, fast_inv_sqrt(squared_sum))
}

#[inline(always)]
pub fn vec_dot<const V: usize>(a: Vec<V>, b: Vec<V>) -> f32 {
    let mut result = 0.0;
    for (a, b) in a.iter().zip(b.iter()) {
        result += a * b;
    }
    result
}

#[inline(always)]
pub fn vec_length<const V: usize>(v: [f32; V]) -> f32 {
    let mut sqsum = 0.0;
    for vs in v.iter() {
        sqsum += vs * vs;
    }
    sqsum.sqrt()
}

#[inline(always)]
pub fn vec_intersects_plane<const V: usize>(
    plane_point: Vec<V>,
    plane_normal: Vec<V>,
    line_start: Vec<V>,
    line_end: Vec<V>,
) -> Vec<V> {
    let d = -vec_dot(plane_normal, plane_point);
    let ad = vec_dot(line_start, plane_normal);
    let bd = vec_dot(line_end, plane_normal);
    let mut td = bd - ad;
    if td == 0.0 {
        td += 0.00001;
    }
    let t = (-d - ad) / td;
    let line_start_to_end = vec_sub_vec(line_end, line_start);
    let line_to_intersect = vec_mul_scalar(line_start_to_end, t);
    vec_add_vec(line_start, line_to_intersect)
}

pub fn triangle_clip_plane(
    plane_point: Vec3,
    plane_normal: Vec3,
    triangle: (Vec3, Vec3, Vec3),
) -> SmallVec<[(Vec3, Vec3, Vec3); 2]> {
    let dist =
        |point: Vec3| -> f32 { vec_dot(plane_normal, point) - vec_dot(plane_normal, plane_point) };

    let mut inside_points: SmallVec<[Vec3; 3]> = smallvec![];
    let mut outside_points: SmallVec<[Vec3; 3]> = smallvec![];

    let d0 = dist(triangle.0);
    let d1 = dist(triangle.1);
    let d2 = dist(triangle.2);

    if d0 >= 0.0 {
        inside_points.push(triangle.0);
    } else {
        outside_points.push(triangle.0)
    }
    if d1 >= 0.0 {
        inside_points.push(triangle.1);
    } else {
        outside_points.push(triangle.1);
    }
    if d2 >= 0.0 {
        inside_points.push(triangle.2);
    } else {
        outside_points.push(triangle.2);
    }

    if inside_points.len() == 3 {
        return smallvec![triangle];
    }

    if inside_points.len() == 1 && outside_points.len() == 2 {
        return smallvec![(
            inside_points[0],
            vec_intersects_plane(
                plane_point,
                plane_normal,
                inside_points[0],
                outside_points[0]
            ),
            vec_intersects_plane(
                plane_point,
                plane_normal,
                inside_points[0],
                outside_points[1]
            )
        )];
    }

    if inside_points.len() == 2 && outside_points.len() == 1 {
        let ot = vec_intersects_plane(
            plane_point,
            plane_normal,
            inside_points[0],
            outside_points[0],
        );
        return smallvec![
            (inside_points[0], inside_points[1], ot),
            (
                inside_points[1],
                ot,
                vec_intersects_plane(
                    plane_point,
                    plane_normal,
                    inside_points[1],
                    outside_points[0]
                )
            )
        ];
    }

    smallvec![]
}

fn fast_inv_sqrt(number: f32) -> f32 {
    let x2 = number * 0.5;
    let y: f32 = number;
    let i: u32 = unsafe { *(&y as *const f32 as *const u32) };
    let i = 0x5f3759df - (i >> 1);
    let y = unsafe { *(&i as *const u32 as *const f32) };
    y * (1.5 - (x2 * y * y))
}
