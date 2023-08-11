use std::io;

use raytracer::Image;

pub(super) fn ppm<T: io::Write>(w: &mut T, image: Image) -> io::Result<()> {
    writeln!(w, "P3\n{} {}\n255", image.width(), image.height())?;

    for pixel in image.data() {
        writeln!(w, "{} {} {}", pixel.r(), pixel.g(), pixel.b())?;
    }

    Ok(())
}
