use super::ray::Ray;
use super::vec3::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord { t, p, normal }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
