use super::Vec3;
use std::{
    fmt::Display,
    ops::{Mul, MulAssign, Neg},
};
use zerocopy::AsBytes;

#[repr(C)]
#[derive(AsBytes, Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    pub const IDENTITY: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    pub fn from_eular(x: f32, y: f32, z: f32) -> Self {
        let half_x = x * 0.5;
        let half_y = y * 0.5;
        let half_z = z * 0.5;

        let sin_x = half_x.sin();
        let cos_x = half_x.cos();
        let sin_y = half_y.sin();
        let cos_y = half_y.cos();
        let sin_z = half_z.sin();
        let cos_z = half_z.cos();

        Self {
            x: sin_x * cos_y * cos_z + cos_x * sin_y * sin_z,
            y: cos_x * sin_y * cos_z - sin_x * cos_y * sin_z,
            z: cos_x * cos_y * sin_z - sin_x * sin_y * cos_z,
            w: cos_x * cos_y * cos_z + sin_x * sin_y * sin_z,
        }
    }

    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        let half_angle = angle * 0.5;
        let s = half_angle.sin();

        Self {
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
            w: half_angle.cos(),
        }
    }

    pub fn normalize(&mut self) -> &mut Self {
        let len = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        if len != 1.0 && len != 0.0 {
            let len = len.sqrt();
            self.x /= len;
            self.y /= len;
            self.z /= len;
            self.w /= len;
        }
        self
    }

    pub fn normalized(self) -> Self {
        let mut result = self;
        result.normalize();
        result
    }

    pub fn conjugate(&mut self) -> &mut Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }

    pub fn conjugated(self) -> Self {
        let mut result = self;
        result.conjugate();
        result
    }

    pub fn invert(&mut self) -> &mut Self {
        self.conjugate().normalize();
        self
    }

    pub fn inverted(self) -> Self {
        let mut result = self;
        result.invert();
        result
    }

    pub fn into_eular(self) -> Vec3 {
        let sinr_cosp = 2.0 * (self.w * self.x + self.y * self.z);
        let cosr_cosp = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);
        let roll = sinr_cosp.atan2(cosr_cosp);

        let sinp = 2.0 * (self.w * self.y - self.z * self.x);
        let pitch = if 1.0 <= sinp.abs() {
            sinp.signum() * std::f32::consts::PI / 2.0
        } else {
            sinp.asin()
        };

        let siny_cosp = 2.0 * (self.w * self.z + self.x * self.y);
        let cosy_cosp = 1.0 - 2.0 * (self.y * self.y + self.z * self.z);
        let yaw = siny_cosp.atan2(cosy_cosp);

        Vec3::new(roll, pitch, yaw)
    }
}

impl Default for Quat {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Mul for Quat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y + self.y * rhs.w + self.z * rhs.x - self.x * rhs.z,
            z: self.w * rhs.z + self.z * rhs.w + self.x * rhs.y - self.y * rhs.x,
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        }
    }
}

impl MulAssign for Quat {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let qvec = Vec3::new(self.x, self.y, self.z);
        let uv = Vec3::cross(qvec, rhs);
        let uuv = Vec3::cross(qvec, uv);
        rhs + ((self.w * uv) + uuv) * 2.0
    }
}

impl Mul<Quat> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Quat) -> Self::Output {
        let qvec = Vec3::new(rhs.x, rhs.y, rhs.z);
        let uv = Vec3::cross(qvec, self);
        let uuv = Vec3::cross(qvec, uv);
        self + ((rhs.w * uv) + uuv) * 2.0
    }
}

impl MulAssign<Quat> for Vec3 {
    fn mul_assign(&mut self, rhs: Quat) {
        *self = *self * rhs;
    }
}

impl Neg for Quat {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
}

impl Display for Quat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let eular = self.into_eular();
        write!(
            f,
            "Quat(x={}, y={}, z={})",
            eular.x.to_degrees(),
            eular.y.to_degrees(),
            eular.z.to_degrees()
        )
    }
}
