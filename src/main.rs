use std::{env::args, time::Instant, str::FromStr};

use structures::{scene::Scene, node::Node, materials::{diffuse::Diffuse, metal::Metal, dielectric::Dielectric}};
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
    let mut output_width = 400;
    let mut output_height = 200;
    let mut aa_sampling = 50;
    let mut ray_recursion_depth = 50;

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

        else if arguments[i] == "-s" {
            if i + 1 >= arguments.len() {
                return Err("Input error: Sampling count promised, but not specified.".to_string());
            } else {
                aa_sampling = match u64::from_str(arguments[i+1].as_str()) {
                    Ok(a) => a,
                    Err(_) => {
                        return Err("Input error: Sampling count not valid.".to_string());
                    }
                }
            }
        }

        else if arguments[i] == "-r" {
            if i + 1 >= arguments.len() {
                return Err("Input error: Recursion depth promised, but not specified.".to_string());
            } else {
                ray_recursion_depth = match u64::from_str(arguments[i+1].as_str()) {
                    Ok(r) => r,
                    Err(_) => {
                        return Err("Input error: Recursion depth invalid.".to_string());
                    }
                }
            }
        }
    }

    return Ok(GeneralInfo {
        out_filename: output_filename,
        out_width: output_width,
        out_height: output_height,
        aa_sampling: aa_sampling,
        ray_recursion: ray_recursion_depth
    });
}

fn ray_color(scene: &Scene, ray: Ray, depth: u64) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let trace_res = scene.trace(&ray, 0.0001, 10000.0);

    if trace_res.is_some() {
        let hit = trace_res.unwrap();
        
        let scatter_opt = hit.material().scatter(&ray, &hit);
        if scatter_opt.is_some() {
            let scatter = scatter_opt.unwrap();
            return &scatter.attenuation * &ray_color(scene, scatter.ray, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let d = ray.get_direction().normalize();
    let t = 0.5 * (d.y + 1.0);
    
    let mut c1 = Color::new(1.0, 1.0, 1.0);
    let mut c2 = Color::new(0.5, 0.7, 1.0);

    c1 = c1 * (1.0 - t);
    c2 = c2 * t;

    return Color::add(&c1, &c2);
}

fn init_scene() -> Scene {
    let mut scene = Scene::new();

    let material1 = Diffuse::new(Color::new(0.8, 0.8, 0.0));
    let material2 = Diffuse::new(Color::new(0.1, 0.2, 0.5));
    //let material3 = Metal::new(Color::new(0.8, 0.8, 0.8), 0.5);
    let material4 = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);
    let material5 = Dielectric::new(1.5);
    let material6 = Dielectric::new(1.5);
    
    let mut node1 = Node::new();
    let sphere1 = Sphere::new(
        Vector3::new(0.0, 0.0, -1.0), 0.5,
        Box::new(material2)
    );
    let mut node2 = Node::new();
    let sphere2 = Sphere::new(
        Vector3::new(0.0, -100.5, -1.0), 100.0,
        Box::new(material1)
    );
    let mut node3 = Node::new();
    let sphere3 = Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0), 0.5,
        Box::new(material5)
    );
    let mut node4 = Node::new();
    let sphere4 = Sphere::new(
        Vector3::new(1.0, 0.0, -1.0), 0.5,
        Box::new(material4)
    );
    let mut node5 = Node::new();
    let sphere5 = Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0), -0.4,
        Box::new(material6)
    );

    node1.set_renderable(Box::new(sphere1));
    node2.set_renderable(Box::new(sphere2));
    node3.set_renderable(Box::new(sphere3));
    node4.set_renderable(Box::new(sphere4));
    node5.set_renderable(Box::new(sphere5));

    scene.add_child(node1);
    scene.add_child(node2);
    scene.add_child(node3);
    scene.add_child(node4);
    scene.add_child(node5);

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

    let progress_chunk = (height as f32 / 100.0) as u64 * 10;
    let mut progress_percentage = 0;

    for h in 0..height {
        if h % progress_chunk == 0 {
            println!("Completed {} %", progress_percentage);
            progress_percentage += 10;
        }
        for w in 0..width {
            let mut c = Color::new(0.0, 0.0, 0.0);
            for _ in 0..info.aa_sampling {
                let ray = camera.get_ray(w, h, width, height);
                let ray_color = ray_color(&scene, ray, info.ray_recursion);
                c = c + ray_color;
            }

            if info.aa_sampling > 0 {
                c = c * (1.0 / info.aa_sampling as f32);
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