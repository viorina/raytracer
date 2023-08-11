mod camera;
mod color;
mod image;
mod ray;
mod scatter;
mod scene;
mod vec3;

pub mod hit;
pub mod render;

pub(crate) use color::Color;
pub(crate) use ray::Ray;

pub use camera::Camera;
pub use image::Image;
pub use scatter::Scatter;
pub use scene::Scene;
pub use vec3::Vec3;
