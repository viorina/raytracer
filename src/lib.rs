mod camera;
mod color;
mod image;
mod primitive;
mod ray;
mod scatter;
mod scene;

pub mod hit;
pub mod render;

pub(crate) use color::Color;
pub(crate) use ray::Ray;

pub use camera::Camera;
pub use image::Image;
pub use scene::Scene;
