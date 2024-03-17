use colored::Colorize;
use image::{self, ImageError, Rgb};
use std::cmp::{max, min};
use std::env;

struct Hsl {
    h: f64,
    s: f64,
    l: f64,
}

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
    let mut result = 0u32;
    for y in 0..size_y {
        for x in 0..size_x {
            let pix = img.get_pixel(x, y);
            println!("{:?}", pix);
        }
    }
}

/*
I refered to
[hsl-rs](https://github.com/killercup/hsl-rs) and
[colorsys.rs](https://github.com/emgyrz/colorsys.rs)
These are MIT Licence.
Thanks to these authors and contributers.
*/
fn rgb_to_hsl(rgb: &Rgb<u8>) -> Hsl {
    let mut h: f64;
    let s: f64; // saturation
    let l: f64; // luminance
    let [r, g, b] = rgb.0;
    let max = max(max(r, g), b);
    let min = min(min(r, g), b);
    let max_min_delta_u8 = max - min;
    // to percentage
    let r = r as f64 / 255_f64;
    let g = g as f64 / 255_f64;
    let b = b as f64 / 255_f64;
    let max = max as f64 / 255_f64;
    let min = min as f64 / 255_f64;
    let max_plus_min = max + min;
    let max_min_delta = max_min_delta_u8 as f64 / 255_f64;
    let l = max_plus_min / 2.0;
    if max_min_delta_u8 == 0 {
        return Hsl {
            h: 0f64,
            s: 0f64,
            l,
        };
    };
    let s = if l > 0.5 {
        max_min_delta / (2.0 - max_plus_min)
    } else {
        max_min_delta / max_plus_min
    };
    let r2 = (((max - r) / 6_f64) + (max_min_delta / 2_f64)) / max_min_delta;
    let g2 = (((max - g) / 6_f64) + (max_min_delta / 2_f64)) / max_min_delta;
    let b2 = (((max - b) / 6_f64) + (max_min_delta / 2_f64)) / max_min_delta;
    h = match max {
        x if x == r => b2 - g2,
        x if x == g => (1_f64 / 3_f64) + r2 - b2,
        _ => (2_f64 / 3_f64) + g2 - r2,
    };
    if h < 0 as f64 {
        h += 1.0;
    } else if h > 1.0 {
        h -= 1.0;
    }
    let h_degrees = (h * 360_f64 * 100_f64).round() / 100_f64;
    Hsl { h: h_degrees, s, l }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    I refered to
    [colorsys.rs](https://github.com/emgyrz/colorsys.rs)
    This is MIT Licence.
    Thanks to these authors and contributers.
    */
    #[test]
    fn test_rgb_to_hsl() {
        let asserts = [
            ((255, 255, 255), (0.0, 0.0, 100.0)),
            ((0, 0, 0), (0.0, 0.0, 0.0)),
            ((215, 231, 236), (194.0, 36.0, 88.0)),
            ((108, 225, 36), (97.0, 76.0, 51.0)),
            ((215, 0, 99), (332.0, 100.0, 42.0)),
            ((10, 10, 10), (0.0, 0.0, 4.0)),
        ];
        asserts.iter().for_each(|t| {
            let rgb = Rgb([t.0 .0, t.0 .1, t.0 .2]);
            let hsl = rgb_to_hsl(&rgb);
            assert_eq!(hsl.h.round(), t.1 .0);
            assert_eq!((hsl.s * 100.0).round(), t.1 .1);
            assert_eq!((hsl.l * 100.0).round(), t.1 .2);
        });
    }
}
