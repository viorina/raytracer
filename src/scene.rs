use std::sync::Arc;

use derive_more::Constructor;
use getset::{CopyGetters, Getters};

use crate::{
    hit::{HitList, Sphere},
    primitive::Vec3,
    scatter::{
        material::{Dielectric, Lambertian, Metal},
        Scatter,
    },
    Camera,
};

#[derive(Constructor, CopyGetters, Getters)]
pub struct Scene {
    #[getset(get_copy = "pub")]
    camera: Camera,
    #[getset(get = "pub")]
    world: HitList,
}

impl Default for Scene {
    fn default() -> Self {
        let camera = Camera::default();

        let side_material: Arc<dyn Scatter> = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.1));
        let right_sphere = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, side_material.clone());
        let left_sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, side_material.clone());

        let central_material: Arc<dyn Scatter> = Arc::new(Dielectric::new(1.5));
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
