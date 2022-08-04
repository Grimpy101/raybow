use crate::structures::{scene::Scene};

pub struct GeneralInfo {
    pub out_filename: String,
    pub out_width: u64,
    pub out_height: u64,
    pub aa_sampling: u64,
    pub ray_recursion: u64,
    pub threads: u64,
    pub animation: bool
}

pub struct RenderInfo<'a> {
    pub width_start: u64,
    pub width_end: u64,
    pub height_start: u64,
    pub height_end: u64,
    pub info: &'a GeneralInfo,
    pub scene: &'a Scene,
    pub frame: u64
}