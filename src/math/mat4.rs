use super::vec4::Vec4;
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Mat4([[f32; 4]; 4]);

impl Mat4 {
    pub const fn identity() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn translation(t: Vec4) -> Self {
        Self([
            [1.0, 0.0, 0.0, t.x()],
            [0.0, 1.0, 0.0, t.y()],
            [0.0, 0.0, 1.0, t.z()],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scaling(s: Vec4) -> Self {
        Self([
            [s.x(), 0.0, 0.0, 0.0],
            [0.0, s.y(), 0.0, 0.0],
            [0.0, 0.0, s.z(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation(axis: Vec4, rad: f32) -> Self {
        let n = axis.normalize();
        let (nx, ny, nz) = (n.x(), n.y(), n.z());
        let c = rad.cos();
        let s = rad.sin();
        let omc = 1.0 - c;
        Self([
            [
                c + nx * nx * omc,
                ny * nx * omc - nz * s,
                nz * nx * omc + ny * s,
                0.0,
            ],
            [
                nx * ny * omc + nz * s,
                c + ny * ny * omc,
                ny * nz * omc - nx * s,
                0.0,
            ],
            [
                nx * nz * omc - ny * s,
                nz * ny * omc + nx * s,
                c + nz * nz * omc,
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Mat4 {
        let mut result = [[0.0f32; 4]; 4];
        for r in 0..4 {
            for c in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.0[r][k] * rhs.0[k][c];
                }
                result[r][c] = sum;
            }
        }
        Mat4(result)
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Vec4 {
        let v = [rhs.x(), rhs.y(), rhs.z(), rhs.w()];
        let mut out = [0.0f32; 4];
        for r in 0..4 {
            let mut sum = 0.0;
            for c in 0..4 {
                sum += self.0[r][c] * v[c];
            }
            out[r] = sum;
        }
        Vec4::new(out[0], out[1], out[2], out[3])
    }
}
