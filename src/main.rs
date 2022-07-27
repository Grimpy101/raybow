use std::{env::args, time::Instant, str::FromStr};

use structures::{scene::Scene, node::Node};
use utils::GeneralInfo;

use crate::{color::Color, math::vector3::Vector3, ray::Ray, structures::{sphere::Sphere, camera::Camera}};

mod math;
mod utils;
mod export;
mod color;
mod ray;
mod structures;

fn get_info_from_args() -> Result<GeneralInfo, String> {
    let arguments: Vec<String> = args().collect();

    let mut output_filename = "out".to_string();
    let mut output_width = 200;
    let mut output_height = 100;
    let mut aa_sampling = 50;

    for i in 0..arguments.len() {
        if arguments[i] == "-o" {
            if i + 1 >= arguments.len() {
                return Err("Input error: Output filename promised, but not specified.".to_string());
            } else {
                output_filename = arguments[i+1].clone();
            }
        }

        else if arguments[i] == "-d" {
            if i + 2 >= arguments.len() {
                return Err("Input error: Dimensions promised, but not specified.".to_string());
            } else {
                output_width = match u64::from_str(arguments[i+1].as_str()) {
                    Ok(w) => w,
                    Err(_) => {
                        return Err("Input error: Width not valid.".to_string());
                    }
                };
                output_height = match u64::from_str(arguments[i+2].as_str()) {
                    Ok(h) => h,
                    Err(_) => {
                        return Err("Input error: Height not valid.".to_string());
                    }
                }
            }
        }

        else if arguments[i] == "-a" {
            if i + 1 >= arguments.len() {
                return Err("Input error: AA sampling count promised, but not specified.".to_string());
            } else {
                aa_sampling = match u64::from_str(arguments[i+1].as_str()) {
                    Ok(a) => a,
                    Err(_) => {
                        return Err("Input error: AA sampling count not valid.".to_string());
                    }
                }
            }
        }
    }

    return Ok(GeneralInfo {
        out_filename: output_filename,
        out_width: output_width,
        out_height: output_height,
        aa_sampling: aa_sampling
    });
}

fn ray_color(scene: &Scene, ray: Ray) -> Color {
    let trace_res = scene.trace(&ray, 0.1, 100.0);
    if trace_res.is_some() {
        let hit = trace_res.unwrap();
        return scene.get_color(&hit);
    }

    let d = ray.get_direction().normalize();
    let t = 0.5 * (d.y + 1.0);
    
    let c1 = Color::new(1.0, 1.0, 1.0);
    let c2 = Color::new(0.5, 0.7, 1.0);

    let c1_interp = Color::scale(&c1, 1.0 - t);
    let c2_interp = Color::scale(&c2, t);

    return Color::add(&c1_interp, &c2_interp);
}

fn init_scene() -> Scene {
    let mut scene = Scene::new();
    
    let mut node1 = Node::new();
    let sphere1 = Sphere::new(
        Vector3::new(0.0, 0.0, -1.0), 0.5
    );
    let mut node2 = Node::new();
    let sphere2 = Sphere::new(
        Vector3::new(0.0, -100.5, -1.0), 100.0
    );

    node1.set_renderable(Box::new(sphere1));
    node2.set_renderable(Box::new(sphere2));

    scene.add_child(node1);
    scene.add_child(node2);

    return scene;
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

    let camera = Camera::new(
        2.0, 2.0 * aspect_ratio,
        1.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0));

    let scene = init_scene();

    for h in 0..height {
        for w in 0..width {
            let mut c = Color::new(0.0, 0.0, 0.0);
            for _ in 0..info.aa_sampling {
                let ray = camera.get_ray(w, h, width, height);
                c = Color::add(&c, &ray_color(&scene, ray));
            }

            if info.aa_sampling > 0 {
                c = Color::scale(&c, 1.0 / info.aa_sampling as f32);
            }
            c.clamp();

            data.push(c);
        }
    }

    export::export_ppm(&info, data);

    let duration = start_time.elapsed();
    println!("Finished in {:?}s.", duration.as_secs_f32());
    println!("Exiting...");
}