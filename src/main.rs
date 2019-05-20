use std::sync::Arc;
use std::{fs, io, path};

use raytracer::camera::Camera;
use raytracer::hit::{HitList, Sphere};
use raytracer::image::Image;
use raytracer::render;
use raytracer::scatter::{Lambertian, Metal, Scatter};
use raytracer::scene::Scene;
use raytracer::vec3::Vec3;

fn main() {
    let result = match parse_args() {
        Destination::Stdout => {
            let mut img = Image::new(200, 100);
            render::render(&mut img, custom_scene());

            let stdout = io::stdout();
            let mut stdoutlock = stdout.lock();
            img.write_ppm(&mut stdoutlock)
        }
        Destination::File(path) => match fs::File::create(path) {
            Ok(mut file) => {
                let mut img = Image::new(200, 100);
                render::render(&mut img, custom_scene());

                img.write_ppm(&mut file)
            }
            Err(error) => Err(error),
        },
    };

    let exit_code = match result {
        Ok(()) => exitcode::OK,
        Err(error) => {
            eprintln!("{}", error);
            exitcode::IOERR
        }
    };
    std::process::exit(exit_code);
}

enum Destination {
    Stdout,
    File(path::PathBuf),
}

fn parse_args() -> Destination {
    let mut args = std::env::args();
    match args.len() {
        1 => Destination::Stdout,
        2 => {
            let filename = args.nth(1).unwrap();
            let path = path::PathBuf::from(filename);
            Destination::File(path)
        }
        _ => {
            eprintln!("Too many arguments");
            std::process::exit(exitcode::USAGE);
        }
    }
}

fn custom_scene() -> Scene {
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, 1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, -2.0, 0.0),
    );

    let side_material: Arc<dyn Scatter> =
        Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.1));
    let right_sphere = Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::clone(&side_material)
    );
    let left_sphere = Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::clone(&side_material)
    );

    let central_material: Arc<dyn Scatter> =
        Arc::new(Lambertian::new(Vec3::new(0.0, 0.2, 0.6)));
    let central_sphere = Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        central_material
    );

    let ground_material: Arc<dyn Scatter> =
        Arc::new(Lambertian::new(Vec3::new(0.3, 0.3, 0.3)));
    let ground = Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        ground_material
    );

    let mut world = HitList::default();
    world.push(Box::new(right_sphere));
    world.push(Box::new(left_sphere));
    world.push(Box::new(central_sphere));
    world.push(Box::new(ground));

    Scene::new(camera, world)
}
