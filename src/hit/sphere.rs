use std::{borrow::Borrow, sync::Arc};

use derive_more::Constructor;

use crate::{
    hit::{Hit, HitRecord},
    Interval, Ray, Scatter, Vec3,
};

#[derive(Constructor)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Arc<dyn Scatter>,
}

impl Hit for Sphere {
    fn hit(&self, ray: Ray, t: Interval) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().squared_length();
        let b = oc.dot(ray.direction());
        let c = oc.squared_length() - self.radius.powi(2);
        let discriminant = b * b - a * c;

        if discriminant <= 0.0 {
            return None;
        }

        let roots = (
            (-b - discriminant.sqrt()) / a,
            (-b + discriminant.sqrt()) / a,
        );

        let t = match (t.surrounds(roots.0), t.surrounds(roots.1)) {
            (true, _) => roots.0,
            (false, true) => roots.1,
            (false, false) => return None,
        };

        let point = ray.point_at_parameter(t);
        let normal = (point - self.center) / self.radius;

        Some(HitRecord::new(t, point, normal, self.material.borrow()))
    }
}
