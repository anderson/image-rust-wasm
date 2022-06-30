pub struct Rgba;

impl Rgba {
    pub const RED: usize = 0;
    pub const GREEN: usize = 1;
    pub const BLUE: usize = 2;
    pub const ALPHA: usize = 3;
}

pub fn get_red(raw_pixel: &[u8]) -> u8 {
    raw_pixel[Rgba::RED]
}

pub fn get_inverted_red(raw_pixel: &[u8]) -> u8 {
    255 - get_red(raw_pixel)
}

pub fn get_green(raw_pixel: &[u8]) -> u8 {
    raw_pixel[Rgba::GREEN]
}

pub fn get_inverted_green(raw_pixel: &[u8]) -> u8 {
    255 - get_green(raw_pixel)
}

pub fn get_blue(raw_pixel: &[u8]) -> u8 {
    raw_pixel[Rgba::BLUE]
}

pub fn get_inverted_blue(raw_pixel: &[u8]) -> u8 {
    255 - get_blue(raw_pixel)
}

pub fn get_avg(raw_pixel: &[u8]) -> u8 {
    let avg =
        (get_red(raw_pixel) as u32 + get_green(raw_pixel) as u32 + get_blue(raw_pixel) as u32) / 3;
    avg as u8
}

pub fn get_weighted(raw_pixel: &[u8], red_w: f32, gree_w: f32, blue_w: f32) -> f32 {
    get_red(raw_pixel) as f32 * red_w
        + get_green(raw_pixel) as f32 * gree_w
        + get_blue(raw_pixel) as f32 * blue_w
}

pub fn set_red(raw_pixel: &mut [u8], value: u8) {
    raw_pixel[Rgba::RED] = value;
}

pub fn set_green(raw_pixel: &mut [u8], value: u8) {
    raw_pixel[Rgba::GREEN] = value;
}

pub fn set_blue(raw_pixel: &mut [u8], value: u8) {
    raw_pixel[Rgba::BLUE] = value;
}
