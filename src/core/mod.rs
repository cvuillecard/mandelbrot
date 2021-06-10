use image::ColorType;
use image::png::PNGEncoder;
use std::str::FromStr;
use num::Complex;
use std::fs::File;

pub mod util;

pub mod argument {
    use super::{Complex, FromStr};

    pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
        match s.find(separator) {
            None => None,
            Some(idx) => {
                match (T::from_str(&s[..idx]), T::from_str(&s[idx + 1..])) {
                    (Ok(l), Ok(r)) => Some((l, r)),
                    _ => None
                }
            }
        }
    }

    pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
        match parse_pair(s, ',') {
            Some((l, r)) => Some(Complex { re: l, im: r }),
            None => None
        }
    }
}

pub mod display {
    use super::{Complex};

    #[allow(dead_code)]
    fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
        let mut z = Complex { re: 0.0, im: 0.0 };

        for i in 0..limit {
            z = z * z + c;
            if z.norm_sqr() > 4.0 {
                return Some(i);
            }
        }
        None
    }

    pub(super) fn pixel_to_complex(surface: (usize, usize), pixel: (usize, usize), top_left_angle: Complex<f64>, bottom_right_angle: Complex<f64>) -> Complex<f64> {
        let (width, height) = (bottom_right_angle.re - top_left_angle.re, top_left_angle.im - bottom_right_angle.im);

        Complex {
            re: top_left_angle.re + pixel.0 as f64 * width / surface.0 as f64,
            im: top_left_angle.im - pixel.1 as f64 * height / surface.1 as f64,
        }
    }

    fn render(pixels: &mut [u8], surface: (usize, usize), top_left_angle: Complex<f64>, bottom_right_angle: Complex<f64>) {
        assert_eq!(pixels.len(), surface.0 * surface.1);

        for x in 0..surface.1 {
            for y in 0..surface.0 {
                let point = pixel_to_complex(surface, (y, x), top_left_angle, bottom_right_angle);
                pixels[x * surface.0 + y] =
                    match self::escape_time(point, 255) {
                        None => 0,
                        Some(count) => 255 - count as u8
                    };
            }
        }
    }

    pub fn exec_render(nb_exetron: usize, pixels: &mut [u8], surface: (usize, usize), top_left_angle: Complex<f64>, bottom_right_angle: Complex<f64>) {
        let nb_line = surface.1 / nb_exetron + 1;

        {
            let lines: Vec<&mut [u8]> = pixels.chunks_mut(nb_line * surface.0).collect();
            crossbeam::scope(|spawner| {
                for (i, line) in lines.into_iter().enumerate() {
                    let top = nb_line * i;
                    let high = line.len() / surface.0;
                    let line_surface = (surface.0, high);
                    let line_top_left = pixel_to_complex(surface, (0, top), top_left_angle, bottom_right_angle);
                    let line_bottom_right = pixel_to_complex(surface, (surface.0, top + high), top_left_angle, bottom_right_angle);

                    spawner.spawn(move || render(line, line_surface, line_top_left, line_bottom_right));
                }
            })
        }
    }
}

pub fn write_img(filename: &str, pixels: &[u8], surface: (usize, usize)) -> Result<(), std::io::Error> {
    let encoder: PNGEncoder<File> = PNGEncoder::new(File::create(filename)?);
    encoder.encode(&pixels, surface.0 as u32, surface.1 as u32, ColorType::Gray(8))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{argument, display, Complex};

    #[test]
    fn test_parse_pair() {
        assert_eq!(argument::parse_pair::<i32>("", 's'), None);
        assert_eq!(argument::parse_pair::<i32>("10", ','), None);
        assert_eq!(argument::parse_pair::<i32>(",10", ','), None);
        assert_eq!(argument::parse_pair::<i32>("10,20", ','), Some((10, 20)));
        assert_eq!(argument::parse_pair::<i32>("10,20xy", ','), None);
        assert_eq!(argument::parse_pair::<i32>("10x20", 'x'), Some((10, 20)));
        assert_eq!(argument::parse_pair::<f64>("1.5x", 'x'), None);
        assert_eq!(argument::parse_pair::<f64>("1.5x3.0", 'x'), Some((1.5, 3.0)));
    }

    #[test]
    fn test_parse_complex() {
        assert_eq!(argument::parse_complex("1.5,3.0"), Some(Complex { re: 1.5, im: 3.0 }));
        assert_eq!(argument::parse_complex("0.035,"), None);
    }

    #[test]
    fn test_pixel_to_complex() {
        assert_eq!(display::pixel_to_complex((100, 100), (25, 75), Complex {re: -1.0, im: 1.0}, Complex{ re: 1.0, im: -1.0}), Complex { re: -0.5, im: -0.5 });
    }
}
