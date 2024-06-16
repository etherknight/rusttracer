use std::f64;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Image
    let width: i32 = 256;
    let height: i32 = 256;

    println!("P3\n{} {}\n255\n", width, height);

    for h in 0..height {
        for w in 0..width {
            let r :f64 = f64::from(w) / (width as f64 - 1.0);
            let g :f64 = f64::from(h) / (height as f64 - 1.0);
            let b :f64 = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}