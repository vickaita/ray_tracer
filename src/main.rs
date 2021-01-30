mod camera;
mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hitable::Hitable;
use ray::Ray;
use sphere::Sphere;
use std::io::{self, Write};
use vec3::Vec3;
use rand;

fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) - Vec3::ones();
        if p.squared_length() < 1.0 {
            break;
        }
    }
    return p;
}

fn color(ray: Ray, world: &dyn Hitable) -> Vec3 {
    if let Some(hit) = world.hit(&ray, 0.0, std::f32::MAX) {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        return 0.5 * color(Ray::new(hit.p, target - hit.p), world);
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3::ones() + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn main() -> io::Result<()> {
    let nx = 800;
    let ny = 400;
    let ns = 100;

    io::stdout().write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;

    let cam: Camera = Camera::new();
    let mut world: Vec<Box<dyn Hitable>> = Vec::new();
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zeros();
            for _ in 0..ns {
                let u = (i as f32 + rand::random::<f32>()) / nx as f32;
                let v = (j as f32 + rand::random::<f32>()) / ny as f32;
                let r = cam.get_ray(u, v);
                col = col + color(r, &world);
            }
            col = col / ns as f32;
            io::stdout().write_all(
                format!("{} {} {}\n", col.r() as i32, col.g() as i32, col.b() as i32).as_bytes(),
            )?;
        }
    }

    Ok(())
}
