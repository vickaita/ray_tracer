use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(self) -> f32 { self.x }
    pub fn y(self) -> f32 { self.y }
    pub fn z(self) -> f32 { self.z }
    pub fn r(self) -> f32 { self.x }
    pub fn g(self) -> f32 { self.y }
    pub fn b(self) -> f32 { self.z }

    pub fn squared_length(self) -> f32 { self.x * self.x + self.y * self.y + self.z * self.z }
    pub fn length(self) -> f32 { self.squared_length().sqrt() }
    // pub fn dot(self, v2: Self) -> f32 { self.x * v2.x + self.y * v2.y + self.z * v2.z }
    // pub fn cross(self, v2: Self) -> Self {
    //     Vec3::new(
    //         self.y * v2.z - self.z * v2.y,
    //         -(self.x * v2.z - self.z * v2.x),
    //         self.x * v2.y - self.y * v2.x)
    // }
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: v.x * self,
            y: v.y * self,
            z: v.z * self,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Self {
        Self {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vec() {
        assert_eq!(
            Vec3::new(1.0, 2.0, -3.0) + Vec3::new(1.0, 0.0, 2.0),
            Vec3::new(2.0, 2.0, -1.0)
        )
    }

    #[test]
    fn test_mul_scalar() {
        assert_eq!(Vec3::new(1.0, 2.0, -3.0) * 5.0, Vec3::new(5.0, 10.0, -15.0))
    }
}
