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
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray.direction().unit_vector().reflect(hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere());
        if scattered.direction().dot(hit.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn schlick(&self, cosine: f32) -> f32 {
        let r0 = ((1.0 - self.refraction_index) / (1.0 + self.refraction_index)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray.direction().reflect(hit.normal);
        let attenuation = Vec3::ones();
        let outward_normal;
        let ni_over_nt;
        let reflect_prob;
        let cosine;
        if ray.direction().dot(hit.normal) > 0.0 {
            outward_normal = -1.0 * hit.normal;
            ni_over_nt = self.refraction_index;
            cosine =
                self.refraction_index * ray.direction().dot(hit.normal) / ray.direction().length();
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.refraction_index;
            cosine = -1.0 * ray.direction().dot(hit.normal) / ray.direction().length();
        }
        if let Some(refracted) = ray.direction().refract(outward_normal, ni_over_nt) {
            reflect_prob = self.schlick(cosine);
            if rand::random::<f32>() < reflect_prob {
                return Some((attenuation, Ray::new(hit.p, reflected)));
            }
            return Some((attenuation, Ray::new(hit.p, refracted)));
        } else {
            return Some((attenuation, Ray::new(hit.p, reflected)));
        }
    }
}
