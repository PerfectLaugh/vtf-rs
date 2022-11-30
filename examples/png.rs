use image::DynamicImage;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;
use vtf::Error;

fn main() -> Result<(), Error> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        panic!("Usage: png <path to vtf file> <destination of new png file>");
    }

    let path = Path::new(&args[1]);
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let vtf = vtf::from_bytes(&mut buf)?;
    let image = vtf.highres_image.decode(0)?;

    // rgb and rgba images we can save directly, for other formats we convert to rgba
    match image {
        DynamicImage::ImageRgb8(_) => image.save(&args[2])?,
        DynamicImage::ImageRgba8(_) => image.save(&args[2])?,
        // DynamicImage::ImageBgra8(_) => image.to_rgba().save(&args[2])?,
        _ => image.to_rgb8().save(&args[2])?,
    };
    Ok(())
}
