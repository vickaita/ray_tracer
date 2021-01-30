use super::hitable::{HitRecord, Hitable};
use super::ray::Ray;

impl Hitable for Vec<Box<dyn Hitable>> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut best: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for h in self.iter() {
            if let Some(hit) = h.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                best = Some(hit);
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {}
