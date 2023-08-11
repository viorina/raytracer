use derive_more::Constructor;

use crate::{
    hit::HitRecord,
    scatter::{Scatter, ScatteredRay},
    Ray, Vec3,
};

use super::utils;

#[derive(Debug, Constructor)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Scatter for Lambertian {
    fn scatter(&self, _: Ray, hit: HitRecord) -> Option<ScatteredRay> {
        let target = hit.normal() + utils::random_in_unit_sphere();
        let scattered = Ray::new(hit.point(), target);

        Some(ScatteredRay::new(scattered, self.albedo))
    }
}
