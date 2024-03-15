use colored::Colorize;
use image::{self, ImageError};
use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let image_path = match args.next() {
        None => {
            panic!(
                "{}",
                r#"
==========================
Image path is not given.
Use $ count_color_in_image "path_to_image"
==========================
"#
                .red()
            );
        }
        Some(path) => path,
    };
    println!("{}", image_path);
    let img = match image::open(&image_path) {
        Ok(img) => img,
        Err(ref err) => {
            if let ImageError::IoError(err) = err {
                if err.kind() == std::io::ErrorKind::NotFound {
                    panic!(
                        "{}\n{:?}",
                        format!("file: [{image_path}] is not found.").red(),
                        err
                    );
                }
                panic!();
            } else {
                panic!(
                    "{}",
                    format!(
                        "Any problem was happened. \"{image_path}\" was can't opened.{:?}",
                        err
                    )
                    .red()
                )
            }
        }
    };
    let img = img.to_rgb8();
    let size_x = img.width();
    let size_y = img.height();
    for y in 0..size_y {
        for x in 0..size_x {
            let pix = img.get_pixel(x, y);
            println!("{:?}", pix);
        }
    }
}
