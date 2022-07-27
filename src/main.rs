use std::{env::args, time::Instant};

use structures::renderable::Renderable;
use utils::GeneralInfo;

use crate::{color::Color, math::vector3::Vector3, ray::Ray, structures::sphere::Sphere};

mod math;
mod utils;
mod export;
mod color;
mod ray;
mod camera;
mod structures;

fn get_info_from_args() -> Result<GeneralInfo, String> {
    let arguments: Vec<String> = args().collect();
    if arguments.len() <= 1 {
        return Err("Error: Input file not provided!".to_string());
    }
    if arguments.len() <= 2 {
        return Err("Error: Output file not provided!".to_string());
    }

    let input = arguments[1].clone();
    let output = arguments[2].clone();
    let w = 255;
    let h = 155;

    return Ok(GeneralInfo {
        in_filename: input,
        out_filename: output,
        out_width: w,
        out_height: h
    });
}

fn ray_color(renderable: &dyn Renderable, ray: Ray) -> Color {
    let trace_res = renderable.trace(&ray, 1.0, 100.0);
    if trace_res.is_some() {
        let hit = trace_res.unwrap();
        return renderable.get_color(&hit);
    }

    let d = ray.get_direction().normalize();
    let t = 0.5 * (d.y + 1.0);
    
    let c1 = Color::new(1.0, 1.0, 1.0);
    let c2 = Color::new(0.5, 0.7, 1.0);

    let c1_interp = Color::scale(&c1, 1.0 - t);
    let c2_interp = Color::scale(&c2, t);

    return Color::add(&c1_interp, &c2_interp);
}

fn main() {
    println!("Starting Raybow...");
    let start_time = Instant::now();

    let info = match get_info_from_args() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut data: Vec<Color> = Vec::new();
    let height = info.out_height;
    let width = info.out_width;

    let aspect_ratio = (info.out_width as f32) / (info.out_height as f32);

    let camera = camera::Camera::new(
        2.0, 2.0 * aspect_ratio,
        1.0,
        Vector3::new(0.0, 0.0, 0.0));

    let sphere = Sphere::new(
        Vector3::new(0.0, 0.0, -1.0), 0.5);

    for h in 0..height {
        for w in 0..width {
            let ray_direction = camera.get_ray_direction(w, h, width, height);
            let ray = Ray::new(camera.get_origin().copy(), ray_direction);

            let c = ray_color(&sphere, ray);

            data.push(c);
        }
    }

    export::export_ppm(&info, data);

    let duration = start_time.elapsed();
    println!("Finished in {:?}s.", duration.as_secs_f32());
    println!("Exiting...");
}