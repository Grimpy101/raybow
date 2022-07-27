use std::fs;

use crate::{utils::GeneralInfo, color::Color};

pub fn export_ppm(info: &GeneralInfo, data: Vec<Color>) {
    let width = info.out_width;
    let height = info.out_height;
    let file_str = format!("{}.ppm", info.out_filename);

    let mut content = format!("P3\n{} {}\n255\n", width, height);
    for d in data {
        let c = d.to_uint8_str();
        content.push_str(&format!("{} ", c));
    }

    match fs::write(&file_str, content) {
        Ok(_) => {
            println!("Written to file {}", file_str);
        },
        Err(e) => {
            eprintln!("Error writing to file: {}", e)
        }
    }
}