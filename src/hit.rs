mod sphere;

use derive_more::Constructor;
use getset::CopyGetters;

use crate::{Interval, Ray, Scatter, Vec3};

pub use sphere::Sphere;

pub trait Hit: Send + Sync {
    fn hit(&self, ray: Ray, t: Interval) -> Option<HitRecord>;
}

#[derive(Clone, Copy, Constructor, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct HitRecord<'a> {
    t: f32,
    point: Vec3,
    normal: Vec3,
    material: &'a dyn Scatter,
}

#[derive(Default)]
pub struct HitList {
    objects: Vec<Box<dyn Hit>>,
}

impl HitList {
    pub fn push(&mut self, object: Box<dyn Hit>) {
        self.objects.push(object)
    }
}

impl Hit for HitList {
    fn hit(&self, ray: Ray, t: Interval) -> Option<HitRecord> {
        let mut closest_t = t.max();
        let mut closest_record = None;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(ray, Interval::new(t.min(), closest_t)) {
                closest_t = record.t();
                closest_record = Some(record);
            }
        }

        closest_record
    }
}
