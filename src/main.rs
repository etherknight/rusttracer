use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
//use std::fmt;

fn main() {
    let image_height = 256;
    let image_width  = 256;
    let image_path = Path::new("image.ppm");
    let image_file_name = image_path.display();

    let mut file = match File::create(&image_path) {
        Err(reason) => panic!("Couldn't open {}: {}", image_file_name, reason),
        Ok(file) => file,
    };

    let mut image = String::from("P3\n").to_owned();
    image.push_str(format!("{} {}\n255\n", image_width, image_height).as_str() );

    for j in 0..image_height {
        for i in 0..image_width {

            let r: f64 = i as f64 / ((image_width - 1) as f64);
            let g: f64 = j as f64 / ((image_height - 1) as f64);
            let b: f64 = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            image.push_str( format!("{} {} {}\n", ir, ig, ib).as_str());
        }
    }

    match file.write_all(image.as_bytes()) {
        Err(reason) => panic!("Couldn't write to {}: {}", image_file_name, reason),
        Ok(_) => println!("Image written to {}", image_file_name) 
    };
}
