use std::f32;

use rand::Rng;

use crate::color::Color;
use crate::hit::{Hit, HitList};
use crate::image::Image;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec3::Vec3;

pub fn render(img: &mut Image, scene: Scene) {
    let n_samples = 100;

    let width = img.width();
    let height = img.height();

    let pixel_color = |row: usize, col: usize| {
        let mut rng = rand::thread_rng();
        let mut intensity = Vec3::default();

        for _ in 0..n_samples {
            let u = (col as f32 + rng.gen::<f32>()) / width as f32;
            let v = (row as f32 + rng.gen::<f32>()) / height as f32;
            let ray = scene.camera().get_ray(u, v);
            intensity += trace(&ray, scene.world(), 0);
        }

        intensity /= n_samples as f32;
        Color::from(intensity.sqrt())
    };

    img.fill(pixel_color);
}

fn trace(ray: &Ray, world: &HitList, depth: i32) -> Vec3 {
    match world.hit(ray, 0.001, f32::MAX) {
        Some(record) => {
            if depth >= 50 {
                return Vec3::default();
            }

            match record.material().scatter(ray, &record) {
                Some(scattered) => {
                    trace(scattered.ray(), world, depth + 1) * scattered.attenuation()
                }
                None => Vec3::default(),
            }
        }
        None => {
            let t = 0.5 * (ray.direction().unit().y() + 1.0);
            Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}
