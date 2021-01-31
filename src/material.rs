use super::hitable::HitRecord;
use super::ray::Ray;
use super::vec3::Vec3;

fn random_in_unit_sphere() -> Vec3 {
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

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        Some((self.albedo, Ray::new(hit.p, target - hit.p)))
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(ray.direction().unit_vector(), hit.normal);
        let scattered = Ray::new(hit.p, reflected);
        if scattered.direction().dot(hit.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}