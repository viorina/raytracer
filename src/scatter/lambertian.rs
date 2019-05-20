use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

use super::{utils, Scatter, ScatteredRay};

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<ScatteredRay> {
        let target = hit.normal() + utils::random_in_unit_sphere();
        let scattered = Ray::new(hit.p(), target);
        Some(ScatteredRay::new(scattered, self.albedo))
    }
}
