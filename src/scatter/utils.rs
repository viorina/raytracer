use rand::distributions::{Distribution, UnitSphereSurface};

use crate::vec3::Vec3;

pub(super) fn random_in_unit_sphere() -> Vec3 {
    let sphere = UnitSphereSurface::new();
    let v = sphere.sample(&mut rand::thread_rng());
    Vec3::new(v[0] as f32, v[1] as f32, v[2] as f32)
}
