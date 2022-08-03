use std::{fs, io::Error};

use crate::color::Color;

use super::media_info::PPMInfo;

pub fn encode(info: &PPMInfo, data: Vec<Color>) -> Result<(), Error> {
    let file_str = format!("{}.ppm", info.filename);
    let mut content = format!("P3\n{} {}\n{}\n", info.width, info.height, info.max_val);
    for d in data {
        let c = d.to_uint8_str();
        content.push_str(&format!("{} ", c));
    }

    fs::write(&file_str, content)
}