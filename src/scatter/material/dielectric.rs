use derive_more::Constructor;
use rand::Rng;

use crate::{
    hit::HitRecord,
    primitive::Vec3,
    scatter::{Scatter, ScatteredRay},
    Ray,
};

#[derive(Debug, Constructor)]
pub struct Dielectric {
    refraction_index: f32,
}

impl Scatter for Dielectric {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<ScatteredRay> {
        let (normal, refraction_ratio) = if hit.contains(ray) {
            (-hit.normal(), self.refraction_index / 1.0)
        } else {
            (hit.normal(), 1.0 / self.refraction_index)
        };
        let ray_direction_unit = ray.direction().unit();

        let is_internal_refracted = {
            let cos_theta = (-ray_direction_unit).dot(normal).min(1.0);
            let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

            refraction_ratio * sin_theta > 1.0
        };
        let reflectance = {
            let cos_theta = (-ray_direction_unit).dot(normal).min(1.0);
            let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);

            r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
        };

        let direction = if is_internal_refracted || reflectance > rand::thread_rng().gen::<f32>() {
            ray_direction_unit.reflect(normal)
        } else {
            ray_direction_unit.refract(normal, refraction_ratio)
        };

        let scattered = Ray::new(hit.point(), direction);
        Some(ScatteredRay::new(scattered, Vec3::ones()))
    }
}
