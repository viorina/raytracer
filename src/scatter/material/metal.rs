use crate::{
    hit::HitRecord,
    primitive::Vec3,
    scatter::{Scatter, ScatteredRay},
    Ray,
};

use super::utils;

#[derive(Debug)]
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

impl Scatter for Metal {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<ScatteredRay> {
        let reflected = ray.direction().unit().reflect(hit.normal());
        let direction = reflected + utils::random_in_unit_sphere() * self.fuzz;
        let scattered = Ray::new(hit.point(), direction);

        if !hit.contains(scattered) {
            return None;
        }

        Some(ScatteredRay::new(scattered, self.albedo))
    }
}