use image::ImageBuffer;
use lazy_static::lazy_static;
use num::complex::Complex;
use rayon::iter::*;
use std::time::Instant;

lazy_static! {
    static ref SETTINGS: Settings = Settings {
        area: (Complex::new(-2.0, -2.0), Complex::new(1.0, 2.0)),
        resolution: (1024*10, 1024*10),
        iterations: 18,
        escape_radius: 3.0
    };
}

fn main() {
    let now = Instant::now();

    let mut img = ImageBuffer::new(SETTINGS.resolution.0, SETTINGS.resolution.1);

    img.enumerate_pixels_mut()
        .par_bridge()
        .into_par_iter()
        .for_each(|(x, y, pixel)| *pixel = image::Rgb(process(x, y)));

    let end = now.elapsed();
    println!(
        "Computed in {}.{} seconds\nSaving...",
        end.as_secs(),
        end.subsec_millis(),
    );
    img.save("mandelbrot.png").unwrap();
    println!("Done");
}

fn process(x: u32, y: u32) -> [u8; 3] {
    let z0 = convert(x, y);
    let mut temp = z0.clone();
    let mut iterations = 0;
    let mut escaped = false;
    for _ in 1..SETTINGS.iterations {
        temp = temp * temp + z0;
        iterations += 1;
        if arg_squared(&temp) > SETTINGS.escape_radius * SETTINGS.escape_radius{
            escaped = true;
            break;
        }
    }
    if !escaped {
        return [0, 0, 0];
    } else {
        for _ in 0..2 {
            temp = temp * temp + z0;
            iterations += 1;
        }
        
        let z = temp;
        let modulus = arg_squared(&z).sqrt();
        let mu = 1.0 + iterations as f64 - (modulus.log10().log10())/2.0_f64.log10();
        //println!("{} {} {} {}", iterations, x, y, mu);
        if mu.is_nan(){
            panic!();
        }
        color_lookup(mu, SETTINGS.iterations + 6)
    }
}

fn convert(x: u32, y: u32) -> Complex<f64> {
    let (x_max, y_max) = SETTINGS.resolution;
    let range = SETTINGS.area.1 - SETTINGS.area.0;
    let x = x as f64 / x_max as f64;
    let y = y as f64 / y_max as f64;
    let scaled_x = x * range.re + SETTINGS.area.0.re;
    let scaled_y = y * range.im + SETTINGS.area.0.im;
    Complex::new(scaled_x, scaled_y)
}  
fn arg_squared(c: &Complex<f64>) -> f64 {
    c.re * c.re + c.im * c.im
}

fn color_lookup(mu: f64, iterations: u32) -> [u8; 3] {
    gradient(mu / iterations as f64)
}

fn gradient(v: f64) -> [u8; 3] {
    [(v * 255.0) as u8, (v * 255.0) as u8, (v * 255.0) as u8]
}

struct Settings {
    area: (Complex<f64>, Complex<f64>),
    resolution: (u32, u32),
    iterations: u32,
    escape_radius: f64,
}
