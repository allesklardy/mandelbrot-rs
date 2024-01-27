use image::{Rgb, RgbImage};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::atomic::AtomicU32;
use std::sync::atomic::{AtomicU64, Ordering::SeqCst};
use std::time::{Instant, UNIX_EPOCH};

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

fn main() {
    let factor : u32 = 1;
    let width = 3840 * factor as u32;
    let height = (width as f64 * (9.0 / 16.0)) as u32;

    let start_x = -2.0;
    let start_y = 1.0;

    let scale_x = 3.0;
    let scale_y = scale_x * 2.0 / 3.0;

    let mut img = RgbImage::new(width, height);

    let total: u64 = width as u64 * height as u64;
    let count = AtomicU32::new(0);
    let last_time = AtomicU64::new(0);

    img.par_enumerate_pixels_mut()
        .into_par_iter()
        .for_each(|(x, y, pixel)| {
            let cx = x as f64 * scale_x / width as f64 + start_x;
            let cy = y as f64 * scale_y / height as f64 - start_y;
            let c = Complex::new(cx, cy);
            let m = mandelbrot(c);
            let colored = colorize(m);
            pixel[0] = colored[0];
            pixel[1] = colored[1];
            pixel[2] = colored[2];

            if count.fetch_add(1, SeqCst) % 10000000 == 0 {
                let now = UNIX_EPOCH.elapsed().unwrap().as_micros() as u64;
                let last = last_time
                    .fetch_update(SeqCst, SeqCst, |_| Some(now))
                    .unwrap();

                let pixels_per_ms = 10000000 as f64 / (now - last) as f64;

                println!(
                    "{:.2}% - {} pixels/us",
                    count.load(SeqCst) as f64 / total as f64 * 100.0,
                    pixels_per_ms
                );
            }
        });
    let save_start_time = Instant::now();
    img.save("mandelbrot.jpg").unwrap();
    println!(
        "Saved in {}ms",
        save_start_time.elapsed().as_millis()
    );
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

fn complex_to_color(c: Complex) -> Rgb<u8> {
    let abs = c.abs().min(4.0).log2() / 2.0;

    Rgb([
        (abs * 255.0) as u8,
        (abs * 255.0) as u8,
        (abs * 255.0) as u8,
    ])
}

fn mandelbrot(c: Complex) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    let mut n: u32 = 0;
    while z.abs() <= 4.0 && n < 1000 {
        z = z.mul(&z).add(&c);
        n += 1;
    }
    n
}
