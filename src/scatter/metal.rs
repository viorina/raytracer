use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

use super::{utils, Scatter, ScatteredRay};

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
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatteredRay> {
        let reflected = ray.direction().unit().reflect(hit.normal());
        let direction = reflected + utils::random_in_unit_sphere() * self.fuzz;

        if direction.dot(hit.normal()) > 0.0 {
            let scattered = Ray::new(hit.p(), direction);
            Some(ScatteredRay::new(scattered, self.albedo))
        } else {
            None
        }
    }
}
