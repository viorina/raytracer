use crate::camera::Camera;
use crate::hit::HitList;

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
