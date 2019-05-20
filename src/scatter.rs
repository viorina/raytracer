use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

mod lambertian;
mod metal;
mod utils;

pub use self::lambertian::Lambertian;
pub use self::metal::Metal;

pub trait Scatter: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatteredRay>;
}

pub struct ScatteredRay {
    ray: Ray,
    attenuation: Vec3,
}

impl ScatteredRay {
    pub fn new(ray: Ray, attenuation: Vec3) -> ScatteredRay {
        ScatteredRay { ray, attenuation }
    }

    pub fn ray(&self) -> &Ray {
        &self.ray
    }

    pub fn attenuation(&self) -> Vec3 {
        self.attenuation
    }
}
