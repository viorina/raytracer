use std::sync::Arc;

use crate::camera::Camera;
use crate::hit::{HitList, Sphere};
use crate::scatter::{Lambertian, Metal, Scatter};
use crate::vec3::Vec3;

pub struct Scene {
    camera: Camera,
    world: HitList,
}

impl Scene {
    pub fn new(camera: Camera, world: HitList) -> Scene {
        Scene { camera, world }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn world(&self) -> &HitList {
        &self.world
    }
}

impl Default for Scene {
    fn default() -> Self {
        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(-2.0, 1.0, -1.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, -2.0, 0.0),
        );

        let side_material: Arc<dyn Scatter> = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.1));
        let right_sphere = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Arc::clone(&side_material));
        let left_sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Arc::clone(&side_material));

        let central_material: Arc<dyn Scatter> =
            Arc::new(Lambertian::new(Vec3::new(0.0, 0.2, 0.6)));
        let central_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, central_material);

        let ground_material: Arc<dyn Scatter> = Arc::new(Lambertian::new(Vec3::new(0.3, 0.3, 0.3)));
        let ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ground_material);

        let mut world = HitList::default();
        world.push(Box::new(right_sphere));
        world.push(Box::new(left_sphere));
        world.push(Box::new(central_sphere));
        world.push(Box::new(ground));

        Scene::new(camera, world)
    }
}
