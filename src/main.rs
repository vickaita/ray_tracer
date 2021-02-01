mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hitable::Hitable;
use material::{Dielectric, Lambertian, Metal};
use rand;
use ray::Ray;
use sphere::Sphere;
use std::io::{self, Write};
use vec3::Vec3;

fn color(ray: Ray, world: &dyn Hitable, depth: i32) -> Vec3 {
    if let Some(hit) = world.hit(&ray, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some((attenuation, scattered)) = hit.material.scatter(&ray, &hit) {
                return attenuation * color(scattered, world, depth + 1);
            }
        }
        return Vec3::zeros();
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3::ones() + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn main() -> io::Result<()> {
    let nx = 400;
    let ny = 200;
    let ns = 100;

    io::stdout().write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;

    let cam: Camera = Camera::new();

    let mut world: Vec<Box<dyn Hitable>> = Vec::new();
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.1)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Dielectric::new(1.5)),
    )));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zeros();
            for _ in 0..ns {
                let u = (i as f32 + rand::random::<f32>()) / nx as f32;
                let v = (j as f32 + rand::random::<f32>()) / ny as f32;
                let r = cam.get_ray(u, v);
                col = col + color(r, &world, 0);
            }
            col = col / ns as f32;
            io::stdout().write_all(
                format!("{} {} {}\n", col.r() as i32, col.g() as i32, col.b() as i32).as_bytes(),
            )?;
        }
    }

    Ok(())
}
