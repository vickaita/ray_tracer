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

    pub fn zeros() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn ones() -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn x(self) -> f32 {
        self.x
    }
    pub fn y(self) -> f32 {
        self.y
    }
    pub fn z(self) -> f32 {
        self.z
    }
    pub fn r(self) -> i32 {
        (self.x.sqrt() * 255.99) as i32
    }
    pub fn g(self) -> i32 {
        (self.y.sqrt() * 255.99) as i32
    }
    pub fn b(self) -> i32 {
        (self.z.sqrt() * 255.99) as i32
    }

    pub fn squared_length(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(self) -> f32 {
        self.squared_length().sqrt()
    }
    pub fn dot(self, v2: Self) -> f32 {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }
    pub fn cross(self, v2: Self) -> Self {
        Vec3::new(
            self.y * v2.z - self.z * v2.y,
            -(self.x * v2.z - self.z * v2.x),
            self.x * v2.y - self.y * v2.x,
        )
    }
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn reflect(self: Vec3, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(self: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
        let uv = self.unit_vector();
        let dt = uv.dot(n);
        let discriminant = 1.0 - ni_over_nt.powf(2.0) * (1.0 - dt.powf(2.0));
        if discriminant > 0.0 {
            return Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt());
        }
        return None;
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut p: Vec3;
        loop {
            p =
                2.0 * Vec3::new(
                    rand::random::<f32>(),
                    rand::random::<f32>(),
                    rand::random::<f32>(),
                ) - Vec3::ones();
            if p.squared_length() < 1.0 {
                break;
            }
        }
        return p;
    }

    pub fn random_in_unit_disc() -> Vec3 {
        let mut p: Vec3;
        loop {
            p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), 0.0)
                - Vec3::new(1.0, 1.0, 0.0);
            if p.dot(p) < 1.0 {
                break;
            }
        }
        return p;
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

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: v.x + self,
            y: v.y + self,
            z: v.z + self,
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

impl Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl Sub<Vec3> for f32 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: v.x - self,
            y: v.y - self,
            z: v.z - self,
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
