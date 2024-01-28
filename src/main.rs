use image::{Rgb, RgbImage};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::spawn;
// use std::sync::atomic::AtomicU32;
// use std::sync::atomic::{AtomicU64, Ordering::SeqCst};
use std::time::Instant;

struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Complex {
        Complex { real, imag }
    }
    fn add(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
    fn mul(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
    fn abs(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
}

fn generate_frame(width: u32, height: u32, middle_x: f64, middle_y: f64, zoom: f64, step: u32) {
    let length_x = 3.0 / zoom;
    let length_y = length_x * 2.0 / 3.0;

    let start_x = middle_x - length_x / 2.0;
    let start_y = middle_y + length_y / 2.0;

    let mut img = RgbImage::new(width, height);

    // let total: u64 = width as u64 * height as u64;
    // let count = AtomicU32::new(0);
    // let last_time = AtomicU64::new(0);

    let start_image_time = Instant::now();
    img.par_enumerate_pixels_mut()
        .into_par_iter()
        .for_each(|(x, y, pixel)| {
            let cx = x as f64 * length_x / width as f64 + start_x;
            let cy = y as f64 * length_y / height as f64 - start_y;
            let c = Complex::new(cx, cy);
            let m = mandelbrot(c);
            let colored = colorize(m);
            pixel[0] = colored[0];
            pixel[1] = colored[1];
            pixel[2] = colored[2];
        });

    println!(
        "Image generated in {}ms",
        start_image_time.elapsed().as_millis()
    );

    spawn(move || {
        let save_start_time = Instant::now();
        img.save(format!("frames/mandelbrot-{}.webp", step)).unwrap();
        println!("Saved in {}ms", save_start_time.elapsed().as_millis());
    });
}

fn main() {
    let factor: u32 = 1;
    let width = 1280 * factor as u32;
    let height = (width as f64 * (9.0 / 16.0)) as u32;

    let middle_x = -1.5;
    let middle_y = -0.0;
    let start_zoom = 1.0;
    let end_zoom = 1_000_000_000_000.0;

    let steps = 6000;

    for i in 0..steps {
        println!("Generating frame {}/{}", i, steps);

        let t = i as f64 / steps as f64;
        let zoom = start_zoom + (end_zoom - start_zoom) * t.powf(2.0 + (steps as f64 / 100.0));

        generate_frame(width, height, middle_x, middle_y, zoom, i);
    }
}

fn colorize(n: u32) -> Rgb<u8> {
    let rgb = match n % 16 {
        0 => [66, 30, 15],
        1 => [25, 7, 26],
        2 => [9, 1, 47],
        3 => [4, 4, 73],
        4 => [0, 7, 100],
        5 => [12, 44, 138],
        6 => [24, 82, 177],
        7 => [57, 125, 209],
        8 => [134, 181, 229],
        9 => [211, 236, 248],
        10 => [241, 233, 191],
        11 => [248, 201, 95],
        12 => [255, 170, 0],
        13 => [204, 128, 0],
        14 => [153, 87, 0],
        15 => [106, 52, 3],
        _ => [0, 0, 0],
    };


    Rgb(rgb)
}

fn mandelbrot(c: Complex) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    let mut n: u32 = 0;
    while z.abs() <= 4.0 && n < 256 {
        z = z.mul(&z).add(&c);
        n += 1;
    }
    n
}
