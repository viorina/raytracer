use derive_more::Constructor;

use crate::{
    hit::HitRecord,
    scatter::{Scatter, ScatteredRay},
    primitive::Vec3,
    Ray,
};

#[derive(Debug, Constructor)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Scatter for Lambertian {
    fn scatter(&self, _: Ray, hit: HitRecord) -> Option<ScatteredRay> {
        let direction = hit.normal() + Vec3::random_in_unit_sphere();
        let scattered = Ray::new(hit.point(), direction);

        Some(ScatteredRay::new(scattered, self.albedo))
    }
}
