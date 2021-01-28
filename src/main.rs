mod ray;
mod vec3;

use ray::Ray;
use std::io::{self, Write};
use vec3::Vec3;

fn hit_sphere(center: Vec3, radius: f32, ray: Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn color(ray: Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction().unit_vector();
    let t: f32 = 0.5 * unit_direction.y() + 1.0;
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() -> io::Result<()> {
    let nx = 800;
    let ny = 400;
    io::stdout().write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(r) * 255.99;
            io::stdout().write_all(
                format!("{} {} {}\n", col.r() as i32, col.g() as i32, col.b() as i32).as_bytes(),
            )?;
        }
    }

    Ok(())
}
