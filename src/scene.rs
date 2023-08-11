use std::sync::Arc;

use derive_more::Constructor;
use getset::{CopyGetters, Getters};

use crate::{
    hit::{HitList, Sphere},
    scatter::{Lambertian, Metal, Scatter},
    Camera, Vec3,
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
