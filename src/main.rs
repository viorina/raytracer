use std::{fs, io, path};

use raytracer::{render, Image, Scene};

fn main() {
    let result = match parse_args() {
        Destination::File(path) => match fs::File::create(path) {
            Ok(mut file) => {
                let mut img = Image::new(200, 100);
                render::render(&mut img, Scene::default());

                img.write_ppm(&mut file)
            }
            Err(error) => Err(error),
        },
        Destination::Stdout => {
            let mut img = Image::new(200, 100);
            render::render(&mut img, Scene::default());

            let stdout = io::stdout();
            let mut stdoutlock = stdout.lock();

            img.write_ppm(&mut stdoutlock)
        }
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

#[derive(Debug)]
enum Destination {
    File(path::PathBuf),
    Stdout,
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
