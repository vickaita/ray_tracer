use super::hitable::{HitRecord, Hitable};
use super::ray::Ray;
use super::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin() - self.center;
        let a: f32 = ray.direction().dot(ray.direction());
        let b: f32 = oc.dot(ray.direction());
        let c: f32 = oc.dot(oc) - self.radius.powf(2.0);
        let discriminant: f32 = b * b - a * c;
        if discriminant > 0.0 {
            let mut tmp = (-b - discriminant.sqrt()) / a;
            if tmp < t_max && tmp > t_min {
                let p = ray.point_at_parameter(tmp);
                return Some(HitRecord::new(tmp, p, (p - self.center) / self.radius));
            }
            tmp = (-b + discriminant.sqrt()) / a;
            if tmp < t_max && tmp > t_min {
                let p = ray.point_at_parameter(tmp);
                return Some(HitRecord::new(tmp, p, (p - self.center) / self.radius));
            }
        }
        return None;
    }
}

// #[cfg(test)]
// mod test {
//     #[test]
//     fn test_hit() {
//         assert_eq!(
//             Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0)
//             .hit(Ray::new()),
//             Some(HitRecord::new())
//         )
//     }
// }
