use std::borrow::Borrow;
use std::sync::Arc;

use crate::ray::Ray;
use crate::scatter::Scatter;
use crate::vec3::Vec3;

use super::{Hit, HitRecord};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Scatter>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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

        let in_bounds = |t: f32| t_min < t && t < t_max;

        let t = match (in_bounds(roots.0), in_bounds(roots.1)) {
            (true, _) => roots.0,
            (false, true) => roots.1,
            (false, false) => return None,
        };

        let point = ray.point_at_parameter(t);
        Some(HitRecord::new(
            t,
            point,
            (point - self.center) / self.radius,
            self.material.borrow(),
        ))
    }
}
