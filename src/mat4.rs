use crate::matrix::Matrix;
use crate::utils::EPSILON;
use crate::vec4::Vec4;
use std::f32;

#[cfg(feature = "Vector3")]
use crate::vec3::Vec3;

pub type Mat4 = [f32; 16];

impl Matrix for Mat4 {
    type MatrixType = Mat4;
    type VectorType = Vec4;

    fn zeros() -> Self {
        [0.; 16]
    }
    fn ones() -> Self {
        [1.; 16]
    }
    fn identity() -> Self {
        [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ]
    }

    fn copy_to(&self, dst: &mut Self) {
        for i in 0..16 {
            dst[i] = self[i];
        }
    }

    fn transpose(&mut self) -> &mut Self {
        let v01 = self[1];
        let v02 = self[2];
        let v03 = self[3];
        let v12 = self[6];
        let v13 = self[7];
        let v23 = self[11];

        self[1] = self[4];
        self[2] = self[8];
        self[3] = self[12];
        self[4] = v01;
        self[6] = self[9];
        self[7] = self[13];
        self[8] = v02;
        self[9] = v12;
        self[11] = self[14];
        self[12] = v03;
        self[13] = v13;
        self[14] = v23;

        self
    }

    fn mul(&mut self, rhs: &Self) -> &mut Self {
        let r00 = rhs[0];
        let r01 = rhs[1];
        let r02 = rhs[2];
        let r03 = rhs[3];
        let r10 = rhs[4];
        let r11 = rhs[5];
        let r12 = rhs[6];
        let r13 = rhs[7];
        let r20 = rhs[8];
        let r21 = rhs[9];
        let r22 = rhs[10];
        let r23 = rhs[11];
        let r30 = rhs[12];
        let r31 = rhs[13];
        let r32 = rhs[14];
        let r33 = rhs[15];

        let mut v0 = self[0];
        let mut v1 = self[1];
        let mut v2 = self[2];
        let mut v3 = self[3];
        self[0] = v0 * r00 + v1 * r10 + v2 * r20 + v3 * r30;
        self[1] = v0 * r01 + v1 * r11 + v2 * r21 + v3 * r31;
        self[2] = v0 * r02 + v1 * r12 + v2 * r22 + v3 * r32;
        self[3] = v0 * r03 + v1 * r13 + v2 * r23 + v3 * r33;

        v0 = self[4];
        v1 = self[5];
        v2 = self[6];
        v3 = self[7];
        self[4] = v0 * r00 + v1 * r10 + v2 * r20 + v3 * r30;
        self[5] = v0 * r01 + v1 * r11 + v2 * r21 + v3 * r31;
        self[6] = v0 * r02 + v1 * r12 + v2 * r22 + v3 * r32;
        self[7] = v0 * r03 + v1 * r13 + v2 * r23 + v3 * r33;

        v0 = self[8];
        v1 = self[9];
        v2 = self[10];
        v3 = self[11];
        self[8] = v0 * r00 + v1 * r10 + v2 * r20 + v3 * r30;
        self[9] = v0 * r01 + v1 * r11 + v2 * r21 + v3 * r31;
        self[10] = v0 * r02 + v1 * r12 + v2 * r22 + v3 * r32;
        self[11] = v0 * r03 + v1 * r13 + v2 * r23 + v3 * r33;

        v0 = self[12];
        v1 = self[13];
        v2 = self[14];
        v3 = self[15];
        self[12] = v0 * r00 + v1 * r10 + v2 * r20 + v3 * r30;
        self[13] = v0 * r01 + v1 * r11 + v2 * r21 + v3 * r31;
        self[14] = v0 * r02 + v1 * r12 + v2 * r22 + v3 * r32;
        self[15] = v0 * r03 + v1 * r13 + v2 * r23 + v3 * r33;

        self
    }
    fn mul_vector(&self, rhs: &[f32]) -> Vec4 {
        debug_assert!(rhs.len() >= 3);
        let x = rhs[0];
        let y = rhs[1];
        let z = rhs[2];
        let w = if rhs.len() > 3 { rhs[3] } else { 1. };

        [
            self[0] * x + self[1] * y + self[2] * z + self[3] * w,
            self[4] * x + self[5] * y + self[6] * z + self[7] * w,
            self[8] * x + self[9] * y + self[10] * z + self[11] * w,
            self[12] * x + self[13] * y + self[14] * z + self[15] * w,
        ]
    }
    fn mul_vector_left(&self, lhs: &[f32]) -> Vec4 {
        debug_assert!(lhs.len() >= 3);
        let x = lhs[0];
        let y = lhs[1];
        let z = lhs[2];
        let w = if lhs.len() > 3 { lhs[3] } else { 1. };
        [
            self[0] * x + self[4] * y + self[8] * z + self[12] * w,
            self[1] * x + self[5] * y + self[9] * z + self[13] * w,
            self[2] * x + self[6] * y + self[10] * z + self[14] * w,
            self[3] * x + self[7] * y + self[11] * z + self[15] * w,
        ]
    }
    fn add(&mut self, rhs: &Self) -> &mut Self {
        for i in 0..16 {
            self[i] += rhs[i];
        }

        self
    }
    fn sub(&mut self, rhs: &Self) -> &mut Self {
        for i in 0..16 {
            self[i] -= rhs[i];
        }

        self
    }

    fn scale(&mut self, factor: f32) -> &mut Self {
        for i in 0..16 {
            self[i] *= factor;
        }

        self
    }

    fn inverse(&mut self) -> Option<&mut Self> {
        let v00 = self[0];
        let v01 = self[1];
        let v02 = self[2];
        let v03 = self[3];
        let v10 = self[4];
        let v11 = self[5];
        let v12 = self[6];
        let v13 = self[7];
        let v20 = self[8];
        let v21 = self[9];
        let v22 = self[10];
        let v23 = self[11];
        let v30 = self[12];
        let v31 = self[13];
        let v32 = self[14];
        let v33 = self[15];

        let tmp00 = v00 * v11 - v01 * v10;
        let tmp01 = v00 * v12 - v02 * v10;
        let tmp02 = v00 * v13 - v03 * v10;
        let tmp03 = v01 * v12 - v02 * v11;
        let tmp04 = v01 * v13 - v03 * v11;
        let tmp05 = v02 * v13 - v03 * v12;
        let tmp06 = v20 * v31 - v21 * v30;
        let tmp07 = v20 * v32 - v22 * v30;
        let tmp08 = v20 * v33 - v23 * v30;
        let tmp09 = v21 * v32 - v22 * v31;
        let tmp10 = v21 * v33 - v23 * v31;
        let tmp11 = v22 * v33 - v23 * v32;

        let det = tmp00 * tmp11 - tmp01 * tmp10 + tmp02 * tmp09 + tmp03 * tmp08 - tmp04 * tmp07
            + tmp05 * tmp06;

        if det.abs() <= EPSILON {
            return None;
        }
        let det_inv = 1.0 / det;

        self[0] = (v11 * tmp11 - v12 * tmp10 + v13 * tmp09) * det_inv;
        self[1] = (v02 * tmp10 - v01 * tmp11 - v03 * tmp09) * det_inv;
        self[2] = (v31 * tmp05 - v32 * tmp04 + v33 * tmp03) * det_inv;
        self[3] = (v22 * tmp04 - v21 * tmp05 - v23 * tmp03) * det_inv;
        self[4] = (v12 * tmp08 - v10 * tmp11 - v13 * tmp07) * det_inv;
        self[5] = (v00 * tmp11 - v02 * tmp08 + v03 * tmp07) * det_inv;
        self[6] = (v32 * tmp02 - v30 * tmp05 - v33 * tmp01) * det_inv;
        self[7] = (v20 * tmp05 - v22 * tmp02 + v23 * tmp01) * det_inv;
        self[8] = (v10 * tmp10 - v11 * tmp08 + v13 * tmp06) * det_inv;
        self[9] = (v01 * tmp08 - v00 * tmp10 - v03 * tmp06) * det_inv;
        self[10] = (v30 * tmp04 - v31 * tmp02 + v33 * tmp00) * det_inv;
        self[11] = (v21 * tmp02 - v20 * tmp04 - v23 * tmp00) * det_inv;
        self[12] = (v11 * tmp07 - v10 * tmp09 - v12 * tmp06) * det_inv;
        self[13] = (v00 * tmp09 - v01 * tmp07 + v02 * tmp06) * det_inv;
        self[14] = (v31 * tmp01 - v30 * tmp03 - v32 * tmp00) * det_inv;
        self[15] = (v20 * tmp03 - v21 * tmp01 + v22 * tmp00) * det_inv;

        Some(self)
    }

    fn det(&self) -> f32 {
        let v00 = self[0];
        let v01 = self[1];
        let v02 = self[2];
        let v03 = self[3];
        let v10 = self[4];
        let v11 = self[5];
        let v12 = self[6];
        let v13 = self[7];
        let v20 = self[8];
        let v21 = self[9];
        let v22 = self[10];
        let v23 = self[11];
        let v30 = self[12];
        let v31 = self[13];
        let v32 = self[14];
        let v33 = self[15];

        let tmp00 = v00 * v11 - v01 * v10;
        let tmp01 = v00 * v12 - v02 * v10;
        let tmp02 = v00 * v13 - v03 * v10;
        let tmp03 = v01 * v12 - v02 * v11;
        let tmp04 = v01 * v13 - v03 * v11;
        let tmp05 = v02 * v13 - v03 * v12;
        let tmp06 = v20 * v31 - v21 * v30;
        let tmp07 = v20 * v32 - v22 * v30;
        let tmp08 = v20 * v33 - v23 * v30;
        let tmp09 = v21 * v32 - v22 * v31;
        let tmp10 = v21 * v33 - v23 * v31;
        let tmp11 = v22 * v33 - v23 * v32;

        tmp00 * tmp11 - tmp01 * tmp10 + tmp02 * tmp09 + tmp03 * tmp08 - tmp04 * tmp07
            + tmp05 * tmp06
    }

    fn adjugate(&mut self) -> &mut Self {
        let v00 = self[0];
        let v01 = self[1];
        let v02 = self[2];
        let v03 = self[3];
        let v10 = self[4];
        let v11 = self[5];
        let v12 = self[6];
        let v13 = self[7];
        let v20 = self[8];
        let v21 = self[9];
        let v22 = self[10];
        let v23 = self[11];
        let v30 = self[12];
        let v31 = self[13];
        let v32 = self[14];
        let v33 = self[15];

        self[0] = v11 * (v22 * v33 - v23 * v32) - v21 * (v12 * v33 - v13 * v32)
            + v31 * (v12 * v23 - v13 * v22);
        self[1] = -(v01 * (v22 * v33 - v23 * v32) - v21 * (v02 * v33 - v03 * v32)
            + v31 * (v02 * v23 - v03 * v22));
        self[2] = v01 * (v12 * v33 - v13 * v32) - v11 * (v02 * v33 - v03 * v32)
            + v31 * (v02 * v13 - v03 * v12);
        self[3] = -(v01 * (v12 * v23 - v13 * v22) - v11 * (v02 * v23 - v03 * v22)
            + v21 * (v02 * v13 - v03 * v12));
        self[4] = -(v10 * (v22 * v33 - v23 * v32) - v20 * (v12 * v33 - v13 * v32)
            + v30 * (v12 * v23 - v13 * v22));
        self[5] = v00 * (v22 * v33 - v23 * v32) - v20 * (v02 * v33 - v03 * v32)
            + v30 * (v02 * v23 - v03 * v22);
        self[6] = -(v00 * (v12 * v33 - v13 * v32) - v10 * (v02 * v33 - v03 * v32)
            + v30 * (v02 * v13 - v03 * v12));
        self[7] = v00 * (v12 * v23 - v13 * v22) - v10 * (v02 * v23 - v03 * v22)
            + v20 * (v02 * v13 - v03 * v12);
        self[8] = v10 * (v21 * v33 - v23 * v31) - v20 * (v11 * v33 - v13 * v31)
            + v30 * (v11 * v23 - v13 * v21);
        self[9] = -(v00 * (v21 * v33 - v23 * v31) - v20 * (v01 * v33 - v03 * v31)
            + v30 * (v01 * v23 - v03 * v21));
        self[10] = v00 * (v11 * v33 - v13 * v31) - v10 * (v01 * v33 - v03 * v31)
            + v30 * (v01 * v13 - v03 * v11);
        self[11] = -(v00 * (v11 * v23 - v13 * v21) - v10 * (v01 * v23 - v03 * v21)
            + v20 * (v01 * v13 - v03 * v11));
        self[12] = -(v10 * (v21 * v32 - v22 * v31) - v20 * (v11 * v32 - v12 * v31)
            + v30 * (v11 * v22 - v12 * v21));
        self[13] = v00 * (v21 * v32 - v22 * v31) - v20 * (v01 * v32 - v02 * v31)
            + v30 * (v01 * v22 - v02 * v21);
        self[14] = -(v00 * (v11 * v32 - v12 * v31) - v10 * (v01 * v32 - v02 * v31)
            + v30 * (v01 * v12 - v02 * v11));
        self[15] = v00 * (v11 * v22 - v12 * v21) - v10 * (v01 * v22 - v02 * v21)
            + v20 * (v01 * v12 - v02 * v11);

        self
    }

    fn translate(&mut self, direction: &[f32]) -> &mut Self {
        debug_assert!(direction.len() >= 3);

        let mut x = direction[0];
        let mut y = direction[1];
        let mut z = direction[2];

        if direction.len() > 3 {
            x /= direction[3];
            y /= direction[3];
            z /= direction[3];
        }

        self[12] += self[0] * x + self[4] * y + self[8] * z;
        self[13] += self[1] * x + self[5] * y + self[9] * z;
        self[14] += self[2] * x + self[6] * y + self[10] * z;
        self[15] += self[3] * x + self[7] * y + self[11] * z;

        self
    }

    fn rotate(&mut self, angle: f32, axis: &[f32]) -> &mut Self {
        debug_assert!(axis.len() >= 3);

        let mut x = axis[0];
        let mut y = axis[1];
        let mut z = axis[2];

        if axis.len() > 3 {
            x /= axis[3];
            y /= axis[3];
            z /= axis[3];
        }

        let len = (x * x + y * y + z * z).sqrt();

        // no rotation around nothing
        if len.abs() <= EPSILON {
            debug_assert!(len.abs() > EPSILON);
            return self;
        }

        x /= len;
        y /= len;
        z /= len;

        let (s, c) = angle.sin_cos();
        let t = 1. - c;

        let v00 = self[0];
        let v01 = self[1];
        let v02 = self[2];
        let v03 = self[3];
        let v10 = self[4];
        let v11 = self[5];
        let v12 = self[6];
        let v13 = self[7];
        let v20 = self[8];
        let v21 = self[9];
        let v22 = self[10];
        let v23 = self[11];

        let rot00 = x * x * t + c;
        let rot01 = y * x * t + z * s;
        let rot02 = z * x * t - y * s;

        let rot10 = x * y * t - z * s;
        let rot11 = y * y * t + c;
        let rot12 = z * y * t + x * s;

        let rot20 = x * z * t + y * s;
        let rot21 = y * z * t - x * s;
        let rot22 = z * z * t + c;

        self[0] = v00 * rot00 + v10 * rot01 + v20 * rot02;
        self[1] = v01 * rot00 + v11 * rot01 + v21 * rot02;
        self[2] = v02 * rot00 + v12 * rot01 + v22 * rot02;
        self[3] = v03 * rot00 + v13 * rot01 + v23 * rot02;
        self[4] = v00 * rot10 + v10 * rot11 + v20 * rot12;
        self[5] = v01 * rot10 + v11 * rot11 + v21 * rot12;
        self[6] = v02 * rot10 + v12 * rot11 + v22 * rot12;
        self[7] = v03 * rot10 + v13 * rot11 + v23 * rot12;
        self[8] = v00 * rot20 + v10 * rot21 + v20 * rot22;
        self[9] = v01 * rot20 + v11 * rot21 + v21 * rot22;
        self[10] = v02 * rot20 + v12 * rot21 + v22 * rot22;
        self[11] = v03 * rot20 + v13 * rot21 + v23 * rot22;

        self
    }
}

pub trait ProjectionMatrix {
    fn create_perspective(fov_y: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4;
    fn create_perspective_from_viewport(
        vp_left: f32,
        vp_right: f32,
        vp_bot: f32,
        vp_top: f32,
        near: f32,
        far: f32,
    ) -> Mat4;

    fn create_orthogonal_from_viewport(
        vp_left: f32,
        vp_right: f32,
        vp_bot: f32,
        vp_top: f32,
        near: f32,
        far: f32,
    ) -> Mat4;
}

impl ProjectionMatrix for Mat4 {
    fn create_perspective(fov_y: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let f = 1. / (fov_y / 2.).tan();
        let nf = 1. / (near - far);
        [
            f / aspect_ratio,
            0.,
            0.,
            0.,
            0.,
            f,
            0.,
            0.,
            0.,
            0.,
            (far + near) * nf,
            -1.,
            0.,
            0.,
            2. * far * near * nf,
            0.,
        ]
    }
    fn create_perspective_from_viewport(
        vp_left: f32,
        vp_right: f32,
        vp_bot: f32,
        vp_top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let wi = 1. / (vp_right - vp_left);
        let hi = 1. / (vp_top - vp_bot);
        let nf = 1. / (near - far);

        [
            near * 2. * wi,
            0.,
            0.,
            0.,
            0.,
            near * 2. * hi,
            0.,
            0.,
            (vp_right + vp_left) * wi,
            (vp_top + vp_bot) * hi,
            (far + near) * nf,
            -1.,
            0.,
            0.,
            far * near * 2. * nf,
            0.,
        ]
    }

    fn create_orthogonal_from_viewport(
        vp_left: f32,
        vp_right: f32,
        vp_bot: f32,
        vp_top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let wi = 1. / (vp_right - vp_left);
        let hi = 1. / (vp_top - vp_bot);
        let nf = 1. / (near - far);

        [
            2. * wi,
            0.,
            0.,
            0.,
            0.,
            2. * hi,
            0.,
            0.,
            0.,
            0.,
            2. * nf,
            0.,
            -(vp_left + vp_right) * wi,
            -(vp_top + vp_bot) * hi,
            (far + near) * nf,
            1.,
        ]
    }
}

#[cfg(feature = "Vector3")]
pub trait ViewMatrix {
    fn look_at_lh(eye: &Vec3, target: &Vec3, up: &Vec3) -> Mat4;
    fn look_at_rh(eye: &Vec3, target: &Vec3, up: &Vec3) -> Mat4;
}

// Just a helper function
#[cfg(feature = "Vector3")]
fn view_matrix(eye: &Vec3, z: &Vec3, up: &Vec3) -> Mat4 {
    use crate::vec3::CrossProduct;
    use crate::Vector;

    let z_mag = z.mag();
    if z_mag < EPSILON {
        // The eye is on the target, do not transform
        return Mat4::identity();
    }

    let zn = z.scale(1.0 / z_mag);

    let x = up.cross(&zn);
    let x_mag = x.mag();
    debug_assert!(x_mag > EPSILON);

    let xn = x.scale(1.0 / x_mag);
    let yn = zn.cross(&xn);
    [
        xn[0],
        yn[0],
        zn[0],
        0.0,
        xn[1],
        yn[1],
        zn[1],
        0.0,
        xn[2],
        yn[2],
        zn[2],
        0.0,
        -xn.dot(eye),
        -yn.dot(eye),
        -zn.dot(eye),
        1.0,
    ]
}

#[cfg(feature = "Vector3")]
impl ViewMatrix for Mat4 {
    fn look_at_lh(eye: &Vec3, target: &Vec3, up: &Vec3) -> Self {
        use crate::Vector;
        let z = target.sub(eye);
        view_matrix(eye, &z, &up)
    }

    fn look_at_rh(eye: &Vec3, target: &Vec3, up: &Vec3) -> Self {
        use crate::Vector;

        let z = eye.sub(target);
        view_matrix(eye, &z, &up)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::almost_eq;

    #[test]
    fn mat4_zeros() {
        let zeros = Mat4::zeros();
        assert!(zeros.iter().all(|&x| x == 0.0));
    }

    #[test]
    fn mat4_ones() {
        let zeros = Mat4::ones();
        assert!(zeros.iter().all(|&x| x == 1.0));
    }

    #[test]
    fn mat4_identity() {
        let i = Mat4::identity();
        assert_eq!(i[0], 1.0);
        assert_eq!(i[1], 0.0);
        assert_eq!(i[2], 0.0);
        assert_eq!(i[3], 0.0);

        assert_eq!(i[4], 0.0);
        assert_eq!(i[5], 1.0);
        assert_eq!(i[6], 0.0);
        assert_eq!(i[7], 0.0);

        assert_eq!(i[8], 0.0);
        assert_eq!(i[9], 0.0);
        assert_eq!(i[10], 1.0);
        assert_eq!(i[11], 0.0);

        assert_eq!(i[12], 0.0);
        assert_eq!(i[13], 0.0);
        assert_eq!(i[14], 0.0);
        assert_eq!(i[15], 1.0);
    }

    #[test]
    fn mat4_copy_to() {
        let mut a = Mat4::zeros();
        let b = Mat4::ones();

        b.copy_to(&mut a);
        assert!(a.iter().all(|&x| x == 1.0));
    }

    #[test]
    fn mat4_transpose() {
        //  1  2  3  4
        //  5  6  7  8
        //  9 10 11 12
        // 13 14 15 16
        //
        // --->
        //
        //  1  5  9 13
        //  2  6 10 14
        //  3  7 11 15
        //  4  8 12 16
        let mut a = [
            1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
        ];
        a.transpose();
        let b = [
            1., 5., 9., 13., 2., 6., 10., 14., 3., 7., 11., 15., 4., 8., 12., 16.,
        ];

        assert_eq!(a, b);
    }

    #[test]
    fn mat4_mul() {
        let mut a = [
            1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
        ];
        let b = [
            11., 12., 13., 14., 15., 16., 17., 18., 19., 20., 21., 22., 23., 24., 25., 26.,
        ];

        let c = [
            190., 200., 210., 220., 462., 488., 514., 540., 734., 776., 818., 860., 1006., 1064.,
            1122., 1180.,
        ];

        assert_eq!(a.mul(&b), &c);
    }

    #[test]
    fn mat4_mul_identity() {
        let a = [
            1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
        ];
        let mut b = Mat4::identity();

        assert_eq!(b.mul(&a), &a);
    }

    #[test]
    fn mat4_add() {
        let mut a = [
            1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
        ];
        let b = [
            11., 12., 13., 14., 15., 16., 17., 18., 19., 20., 21., 22., 23., 24., 25., 26.,
        ];

        let c = [
            12., 14., 16., 18., 20., 22., 24., 26., 28., 30., 32., 34., 36., 38., 40., 42.,
        ];

        assert_eq!(a.add(&b), &c);
    }

    #[test]
    fn mat4_sub() {
        let mut a = [
            16., 15., 14., 13., 12., 11., 10., 9., 8., 7., 6., 5., 4., 3., 2., 1.,
        ];
        let b = [
            11., 12., 13., 14., 15., 16., 17., 18., 19., 20., 21., 22., 23., 24., 25., 26.,
        ];

        let c = [
            5., 3., 1., -1., -3., -5., -7., -9., -11., -13., -15., -17., -19., -21., -23., -25.,
        ];

        assert_eq!(a.sub(&b), &c);
    }

    #[test]
    fn mat4_scale() {
        let mut a = [
            16., 15., 14., 13., 12., 11., 10., 9., 8., 7., 6., 5., 4., 3., 2., 1.,
        ];
        let b = [
            32., 30., 28., 26., 24., 22., 20., 18., 16., 14., 12., 10., 8., 6., 4., 2.,
        ];

        assert_eq!(a.scale(2.0), &b);
    }

    #[test]
    fn mat4_inverse_valid() {
        let mut a = [
            3., 4., 1., 2., 3., 6., 10., 12., 2., 7., 3., 14., 16., 4., 8., 18.,
        ];
        let b = a.clone();

        a.inverse().expect("Inverse should exist");

        let inv = [
            0.0976342, -0.0401802, -0.0548254, 0.0585805, 0.2482163, 0.0093879, 0.0221555,
            -0.0510702, -0.0168982, 0.1415697, -0.1058956, -0.0101389, -0.1344348, -0.0292903,
            0.0908750, 0.0193391,
        ];

        assert!(almost_eq(&inv, &a));

        assert!(almost_eq(a.mul(&b), &Mat4::identity()));
    }

    #[test]
    fn mat4_inverse_invalid() {
        let mut a = [
            1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
        ];
        assert_eq!(a.inverse(), None);
    }

    #[test]
    fn mat4_det() {
        let a = [
            3., 4., 1., 2., 3., 6., 10., 12., 2., 7., 3., 14., 16., 4., 8., 18.,
        ];
        assert_eq!(a.det(), -5326.0);
    }

    #[test]
    fn mat4_adjugate() {
        let mut a = [
            3., 4., 1., 2., 3., 6., 10., 12., 2., 7., 3., 14., 16., 4., 8., 18.,
        ];
        let b = [
            -520., 214., 292., -312., -1322., -50., -118., 272., 90., -754., 564., 54., 716., 156.,
            -484., -103.,
        ];
        assert_eq!(a.adjugate(), &b);
    }

    #[test]
    fn mat4_mul_vector() {
        let a = [
            1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
        ];
        let b = [17., 18., 19., 20.];

        let c = a.mul_vector(&b);
        assert_eq!(c, [190., 486., 782., 1078.]);
    }

    #[test]
    fn mat4_mul_vector_left() {
        let a = [
            1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
        ];
        let b = [17., 18., 19., 20.];

        let c = a.mul_vector_left(&b);
        assert_eq!(c, [538., 612., 686., 760.]);
    }

    #[test]
    fn mat4_translate() {
        let d = [3., -5., 7., 1.];
        let mut m = Mat4::identity();
        m.translate(&d);

        let a = [-3., 5., -7., 1.];
        assert_eq!(m.mul_vector_left(&a), [0., 0., 0., 1.]);
    }

    #[test]
    fn mat4_rotate_x() {
        let mut m = Mat4::identity();
        m.rotate(f32::consts::FRAC_PI_2, &[1., 0., 0., 1.]);
        let v = [-1., 3., 5., 1.];

        let r = m.mul_vector_left(&v);
        assert!(almost_eq(&r, &[-1., -5., 3., 1.]));
    }

    #[test]
    fn mat4_rotate_y() {
        let mut m = Mat4::identity();
        m.rotate(f32::consts::FRAC_PI_2, &[0., 1., 0., 1.]);
        let v = [-1., 3., 5., 1.];

        let r = m.mul_vector_left(&v);
        assert!(almost_eq(&r, &[5., 3., 1., 1.]));
    }

    #[test]
    fn mat4_rotate_z() {
        let mut m = Mat4::identity();
        m.rotate(f32::consts::FRAC_PI_2, &[0., 0., 1., 1.]);
        let v = [-1., 3., 5., 1.];

        let r = m.mul_vector_left(&v);
        assert!(almost_eq(&r, &[-3., -1., 5., 1.]));
    }

    #[test]
    fn mat4_look_at_rh() {
        let eye = [0., 0., 1.];
        let target = [0., 0., 0.];
        let up = [0., 1., 0.];

        let mut expected_view = Mat4::identity();
        expected_view[14] = -1.;

        assert_eq!(Mat4::look_at_rh(&eye, &target, &up), expected_view);

        // Test case where the eye is on the target
        assert_eq!(
            Mat4::look_at_rh(&[0., 0., 0.], &target, &up),
            Mat4::identity()
        );
    }

    #[test]
    fn mat4_look_at_lh() {
        let eye = [0., 0., 1.];
        let target = [0., 0., 0.];
        let up = [0., 1., 0.];

        let mut expected_view = Mat4::identity();
        expected_view[0] = -1.;
        expected_view[10] = -1.;
        expected_view[14] = 1.;

        assert_eq!(Mat4::look_at_lh(&eye, &target, &up), expected_view);

        // Test case where the eye is on the target
        assert_eq!(
            Mat4::look_at_rh(&[0., 0., 0.], &target, &up),
            Mat4::identity()
        );
    }
}
