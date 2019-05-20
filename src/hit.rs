use crate::ray::Ray;
use crate::scatter::Scatter;
use crate::vec3::Vec3;

mod sphere;

pub use self::sphere::Sphere;

pub trait Hit: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord<'a> {
    t: f32,
    point: Vec3,
    normal: Vec3,
    material: &'a dyn Scatter,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, point: Vec3, normal: Vec3, material: &dyn Scatter) -> HitRecord {
        HitRecord {
            t,
            point,
            normal,
            material,
        }
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn p(&self) -> Vec3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn material(&self) -> &dyn Scatter {
        self.material
    }
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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_t = t_max;
        let mut closest_record = None;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(ray, t_min, closest_t) {
                closest_t = record.t();
                closest_record = Some(record);
            }
        }
        closest_record
    }
}
