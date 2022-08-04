use std::{env::args, time::Instant, str::FromStr, fs};

use animation::animation::{AnimationChannel, AnimationKey, Interpolation};
use crossbeam::{thread, channel::unbounded};
use math::vector2::Vector2;
use media::{ppm, media_info::PPMInfo};
use structures::{scene::Scene, node::Node, materials::{diffuse::Diffuse, metal::Metal, dielectric::Dielectric}, material::Material};
use utils::GeneralInfo;

use crate::{color::Color, math::vector3::Vector3, ray::Ray, structures::{sphere::Sphere, camera::Camera}};

mod math;
mod utils;
mod color;
mod ray;
mod structures;
mod media;
mod animation;

fn get_info_from_args() -> Result<GeneralInfo, String> {
    let arguments: Vec<String> = args().collect();

    let mut output_filename = "out".to_string();
    let mut output_width = 400;
    let mut output_height = 200;
    let mut aa_sampling = 50;
    let mut ray_recursion_depth = 50;
    let mut threads = 10;
    let mut animation = false;

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

        else if arguments[i] == "-t" {
            if i + 1 >= arguments.len() {
                return Err("Input error: Threads number promised, but not specified.".to_string());
            } else {
                threads = match u64::from_str(arguments[i+1].as_str()) {
                    Ok(r) => r,
                    Err(_) => {
                        return Err("Input error: Threads number invalid.".to_string());
                    }
                }
            }
        }

        else if arguments[i] == "-a" {
            animation = true;
        }
    }

    return Ok(GeneralInfo {
        out_filename: output_filename,
        out_width: output_width,
        out_height: output_height,
        aa_sampling: aa_sampling,
        ray_recursion: ray_recursion_depth,
        threads: threads,
        animation: animation
    });
}

fn ray_color(scene: &Scene, ray: Ray, depth: u64, f: f32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let trace_res = scene.trace(&ray, 0.0001, 10000.0, f);

    if trace_res.is_some() {
        let hit = trace_res.unwrap();
        
        let scatter_opt = hit.material().scatter(&ray, &hit);
        if scatter_opt.is_some() {
            let scatter = scatter_opt.unwrap();
            return &scatter.attenuation * &ray_color(scene, scatter.ray, depth - 1, f);
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

fn test_scene() -> Scene {
    let mut scene = Scene::new();

    for i in -11..11 {
        for j in -11..11 {
            let x = i as f32 + 0.9 * rand::random::<f32>();
            let y = 0.2;
            let z = j as f32 + 0.9 * rand::random::<f32>();

            let material_number: f32 = rand::random();
            let material: Box<dyn Material + Send + Sync> = if material_number < 0.8 {
                Box::new(Diffuse::new(
                    Color::new(rand::random(), rand::random(), rand::random())
                ))
            } else if material_number < 0.95 {
                Box::new(Metal::new(
                    Color::new(rand::random(), rand::random(), rand::random()),
                    0.3
                ))
            } else {
                Box::new(Dielectric::new(1.5))
            };

            let mut sphere = Sphere::new(
                Vector3::new(x, y, z), 0.25,
                material
            );
            let mut animation_rad = AnimationChannel::new();
            let mut anim_key1 = AnimationKey::new(0.0, 0.0);
            let mut anim_key2 = AnimationKey::new(5.0, 0.2);
            anim_key1.change_interpolation(Interpolation::Linear);
            anim_key2.change_interpolation(Interpolation::Linear);
            animation_rad.add_key(anim_key1);
            animation_rad.add_key(anim_key2);
            sphere.add_animation_channel("radius".to_string(), animation_rad);

            let mut node = Node::new();
            node.set_renderable(Box::new(sphere));
            scene.add_child(node);
        }
    }

    let ground_material = Metal::new(
        Color::new(0.5, 0.5, 0.5), 1.0
    );
    let ground = Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0), 1000.0,
        Box::new(ground_material)
    );
    let mut ground_node = Node::new();
    ground_node.set_renderable(Box::new(ground));
    scene.add_child(ground_node);

    return scene;
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
    let mut sphere1 = Sphere::new(
        Vector3::new(0.0, 0.0, -1.0), 0.5,
        Box::new(material2)
    );
    let mut node2 = Node::new();
    let sphere2 = Sphere::new(
        Vector3::new(0.0, -100.5, -1.0), 100.0,
        Box::new(material1)
    );
    let mut node3 = Node::new();
    let mut sphere3 = Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0), 0.5,
        Box::new(material5)
    );
    let mut node4 = Node::new();
    let sphere4 = Sphere::new(
        Vector3::new(1.0, 0.0, -1.0), 0.5,
        Box::new(material4)
    );
    /*let mut node5 = Node::new();
    let mut sphere5 = Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0), -0.4,
        Box::new(material6)
    );*/

    let mut animation_rad = AnimationChannel::new();
    let mut anim_key1 = AnimationKey::new(1.0, 0.0);
    let mut anim_key2 = AnimationKey::new(5.0, 0.5);
    anim_key1.change_interpolation(Interpolation::Linear);
    anim_key2.change_interpolation(Interpolation::Linear);
    animation_rad.add_key(anim_key1);
    animation_rad.add_key(anim_key2);

    let mut animation_pos_y = AnimationChannel::new();
    let mut anim_key1 = AnimationKey::new(1.0, 0.0);
    let mut anim_key2 = AnimationKey::new(12.0, 1.0);
    let mut anim_key3 = AnimationKey::new(23.0, 0.0);
    anim_key1.change_interpolation(Interpolation::Bezier(
        vec![Vector2::new(0.0, 0.0), Vector2::new(3.0, 0.75)]
    ));
    anim_key3.change_interpolation(Interpolation::Bezier(
        vec![Vector2::new(21.0, 0.75), Vector2::new(23.0, 0.0)]
    ));
    anim_key2.change_interpolation(Interpolation::Bezier(
        vec![Vector2::new(6.0, 1.0), Vector2::new(18.0, 1.0)]
    ));
    animation_pos_y.add_key(anim_key1);
    animation_pos_y.add_key(anim_key2);
    animation_pos_y.add_key(anim_key3);

    sphere1.add_animation_channel("radius".to_string(), animation_rad);
    sphere3.add_animation_channel("center_y".to_string(), animation_pos_y);

    node1.set_renderable(Box::new(sphere1));
    node2.set_renderable(Box::new(sphere2));
    node3.set_renderable(Box::new(sphere3));
    node4.set_renderable(Box::new(sphere4));
    //node5.set_renderable(Box::new(sphere5));

    scene.add_child(node1);
    scene.add_child(node2);
    scene.add_child(node3);
    scene.add_child(node4);
    //scene.add_child(node5);

    return scene;
}

fn render(w_start: u64, w_end: u64, h_start: u64,
    h_end: u64, info: &GeneralInfo, camera: &Camera, scene: &Scene, f: f32) -> Vec<Color> {
    let mut data: Vec<Color> = Vec::new();
    for h in h_start..h_end {
        for w in w_start..w_end {
            let mut c = Color::new(0.0, 0.0, 0.0);
            for _ in 0..info.aa_sampling {
                let ray = camera.get_ray(w, h, info.out_width, info.out_height);
                let ray_color = ray_color(&scene, ray, info.ray_recursion, f);
                c = c + ray_color;
            }

            if info.aa_sampling > 0 {
                c = c * (1.0 / info.aa_sampling as f32);
            }
            c.clamp();
            data.push(c);
        }
    }
    return data;
}

fn render_still(info: &GeneralInfo, camera: &Camera, scene: &Scene, frame: u64) -> Result<(), String> {
    let height = info.out_height;
    let width = info.out_width;
    let mut data: Vec<Color> = Vec::new();
    for _ in 0..height*width {
        data.push(Color::new(0.0, 0.0, 0.0));
    }

    let (st, rt) = unbounded();
    let mut receivers = Vec::new();

    thread::scope(|s| {
        for i in 0..info.threads {
            let thread_share = height / info.threads;
            let min_height = i*thread_share;
            let max_height = if i < (info.threads - 1) {
                min_height + thread_share
            } else {
                height
            };
            
            let min_width = 0;
            let max_width = width;

            let t_info = &info;
            let t_camera = &camera;
            let t_scene = &scene;

            let st_clone = st.clone();
            let rcv_clone = rt.clone();
            receivers.push(rcv_clone);

            s.spawn(move |_| {
                //println!("Rows {}-{} started", min_height, max_height);
                let data_part = render(min_width, max_width, min_height, max_height,
                    t_info, t_camera, t_scene, frame as f32);
                //println!("Rows {}-{} finished", min_height, max_height);
                let msg = (min_height, max_height, data_part);
                st_clone.send(msg).unwrap();
            });
        }

        for receiver in receivers {
            let (h1, _, d) = receiver.recv().unwrap();
            let start = (h1*width) as usize;
            for i in 0..d.len() {
                data[i+start] = d[i].copy();
            }
        }
    }).unwrap();

    let ppm_info = PPMInfo {
        filename: info.out_filename.clone(),
        width: info.out_width,
        height: info.out_height,
        max_val: 255,
    };

    match ppm::encode(&ppm_info, data) {
        Ok(_) => {
            return Ok(());
        },
        Err(e) => {
            return Err(format!("Error outputing: {}", e))
        }
    };
}

fn render_animation(info: &GeneralInfo, camera: &Camera, scene: &Scene, start_frame: u64, end_frame: u64) -> Result<(), String> {
    let height = info.out_height;
    let width = info.out_width;
    let mut data: Vec<Color> = Vec::new();
    for _ in 0..height*width {
        data.push(Color::new(0.0, 0.0, 0.0));
    }

    match fs::create_dir_all(info.out_filename.clone()) {
        Ok(_) => {},
        Err(e) => {
            return Err(format!("Error creating directory: {}", e));
        }
    };

    for frame in start_frame..(end_frame+1) {
        println!("Rendering frame {}...", frame);
        let temp_info = GeneralInfo {
            out_filename: format!("{}/{}_{}",info.out_filename, info.out_filename, frame),
            out_width: info.out_width,
            out_height: info.out_height,
            aa_sampling: info.aa_sampling,
            ray_recursion: info.ray_recursion,
            threads: info.threads,
            animation: info.animation
        };

        match render_still(&temp_info, camera, scene, frame) {
            Ok(_) => {},
            Err(e) => {
                return Err(format!("Error rendering: {}", e));
            }
        };
    }

    return Ok(());
}

fn main() {
    println!("Starting Raybow...");
    let info_start_time = Instant::now();

    let info = match get_info_from_args() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let height = info.out_height;
    let width = info.out_width;
    let mut data: Vec<Color> = Vec::new();
    for _ in 0..height*width {
        data.push(Color::new(0.0, 0.0, 0.0));
    }

    let aspect_ratio = (info.out_width as f32) / (info.out_height as f32);
    let start_frame = 1;
    let end_frame   = 23;

    let camera = Camera::new(
        aspect_ratio,
        30.0,
        Vector3::new(4.0, 1.0, 4.0),
        Vector3::new(-10.0, 40.0, 0.0),
        5.7, 0.1);

    let scene = init_scene();

    if info.animation {
        match render_animation(&info, &camera, &scene, start_frame, end_frame) {
            Ok(_) => {
                println!("Rendering finished successfully.");
            },
            Err(e) => {
                println!("Error rendering animation: {}", e);
            }
        }
    } else {
        match render_still(&info, &camera, &scene, 1) {
            Ok(_) => {
                println!("Rendering finished successfully.");
            },
            Err(e) => {
                println!("Error rendering still: {}", e);
            },
        }
    }

    let info_duration = info_start_time.elapsed();
    println!("Finished in {:?}s.", info_duration.as_secs_f32());
    println!("Exiting...");
}