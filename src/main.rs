mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vec3;

use hitable::Hitable;
use ray::Ray;
use sphere::Sphere;
use std::io::{self, Write};
use vec3::Vec3;

fn color(ray: Ray, world: &dyn Hitable) -> Vec3 {
    if let Some(hit) = world.hit(&ray, 0.0, std::f32::MAX) {
        return (hit.normal + 1.0) * 0.5
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn main() -> io::Result<()> {
    let nx = 800;
    let ny = 400;
    io::stdout().write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let mut world: Vec<Box<dyn Hitable>> = Vec::new();
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    // let world = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(r, &world);
            io::stdout().write_all(
                format!("{} {} {}\n", col.r() as i32, col.g() as i32, col.b() as i32).as_bytes(),
            )?;
        }
    }

    Ok(())
}
