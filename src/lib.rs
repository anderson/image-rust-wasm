use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

#[cfg(not(feature = "parallel"))]
use std::slice::ChunksMut;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "parallel")]
use rayon::slice::ChunksMut;

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

mod color;

const PIXEL_SIZE: usize = 4;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct ImageWasm {
    raw_pixels: Vec<u8>,
}

#[cfg(not(feature = "parallel"))]
fn get_chunked_raw_pixels(img: &mut ImageWasm) -> ChunksMut<u8> {
    img.raw_pixels.chunks_mut(PIXEL_SIZE)
}

#[cfg(feature = "parallel")]
fn get_chunked_raw_pixels(img: &mut ImageWasm) -> ChunksMut<u8> {
    img.raw_pixels.par_chunks_mut(PIXEL_SIZE)
}

#[wasm_bindgen(js_name = getRawImageData)]
pub fn get_raw_image_data(img: &ImageWasm) -> Clamped<Vec<u8>> {
    Clamped(img.raw_pixels.clone())
}

#[wasm_bindgen(js_name = openImage)]
pub fn open_image(image_data: ImageData) -> ImageWasm {
    set_panic_hook();

    let raw_pixels = image_data.data().to_vec();

    ImageWasm { raw_pixels }
}

#[wasm_bindgen(js_name = putImageData)]
pub fn put_image_data(
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    new_image: ImageWasm,
) {
    let raw_pixels = new_image.raw_pixels;
    let new_img_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&raw_pixels),
        canvas.width(),
        canvas.height(),
    );

    ctx.put_image_data(&new_img_data.unwrap(), 0.0, 0.0)
        .unwrap();
}

#[wasm_bindgen]
pub fn grayscale(img: &mut ImageWasm) {
    let chunked_raw_pixels = get_chunked_raw_pixels(img);

    chunked_raw_pixels.for_each(|raw_pixel| {
        let avg = color::get_avg(raw_pixel);
        color::set_red(raw_pixel, avg);
        color::set_green(raw_pixel, avg);
        color::set_blue(raw_pixel, avg);
    });
}

fn remove_color(img: &mut ImageWasm, color_idx: usize) {
    let chunked_raw_pixels = get_chunked_raw_pixels(img);

    chunked_raw_pixels.for_each(|raw_pixel| {
        raw_pixel[color_idx] = 0;
    });
}

#[wasm_bindgen(js_name = removeRed)]
pub fn remove_red(img: &mut ImageWasm) {
    remove_color(img, color::Rgba::RED);
}

#[wasm_bindgen(js_name = removeGreen)]
pub fn remove_green(img: &mut ImageWasm) {
    remove_color(img, color::Rgba::GREEN);
}

#[wasm_bindgen(js_name = removeBlue)]
pub fn remove_blue(img: &mut ImageWasm) {
    remove_color(img, color::Rgba::BLUE);
}

fn swap_color(img: &mut ImageWasm, color_idx_1: usize, color_idx_2: usize) {
    let chunked_raw_pixels = get_chunked_raw_pixels(img);

    chunked_raw_pixels.for_each(|raw_pixel| raw_pixel.swap(color_idx_1, color_idx_2));
}

#[wasm_bindgen(js_name = swapRedBlue)]
pub fn swap_red_blue(img: &mut ImageWasm) {
    swap_color(img, color::Rgba::RED, color::Rgba::BLUE);
}

#[wasm_bindgen(js_name = swapRedGreen)]
pub fn swap_red_green(img: &mut ImageWasm) {
    swap_color(img, color::Rgba::RED, color::Rgba::GREEN);
}

#[wasm_bindgen(js_name = swapBlueGreen)]
pub fn swap_blue_green(img: &mut ImageWasm) {
    swap_color(img, color::Rgba::BLUE, color::Rgba::GREEN);
}

#[wasm_bindgen(js_name = swapAlphaRed)]
pub fn swap_alpha_red(img: &mut ImageWasm) {
    swap_color(img, color::Rgba::ALPHA, color::Rgba::RED);
}

#[wasm_bindgen(js_name = swapAlphaGreen)]
pub fn swap_alpha_green(img: &mut ImageWasm) {
    swap_color(img, color::Rgba::ALPHA, color::Rgba::GREEN);
}

#[wasm_bindgen(js_name = swapAlphaBlue)]
pub fn swap_alpha_blue(img: &mut ImageWasm) {
    swap_color(img, color::Rgba::ALPHA, color::Rgba::BLUE);
}

#[wasm_bindgen]
pub fn sepia(img: &mut ImageWasm) {
    let chunked_raw_pixels = get_chunked_raw_pixels(img);

    chunked_raw_pixels.for_each(|raw_pixel| {
        let weight = color::get_weighted(raw_pixel, 0.3, 0.59, 0.11);

        let new_red = if weight as u32 + 100 < 255 {
            weight as u8 + 100
        } else {
            255
        };

        let new_green = if weight as u32 + 50 < 255 {
            weight as u8 + 50
        } else {
            255
        };

        color::set_red(raw_pixel, new_red);
        color::set_green(raw_pixel, new_green);
    });
}

#[wasm_bindgen]
pub fn invert(img: &mut ImageWasm) {
    let chunked_raw_pixels = get_chunked_raw_pixels(img);

    chunked_raw_pixels.for_each(|raw_pixel| {
        let new_red = color::get_inverted_red(raw_pixel);
        let new_green = color::get_inverted_green(raw_pixel);
        let new_blue = color::get_inverted_blue(raw_pixel);

        color::set_red(raw_pixel, new_red);
        color::set_green(raw_pixel, new_green);
        color::set_blue(raw_pixel, new_blue);
    });
}

#[wasm_bindgen]
pub fn redden(img: &mut ImageWasm) {
    let chunked_raw_pixels = get_chunked_raw_pixels(img);

    chunked_raw_pixels.for_each(|raw_pixel| {
        let new_red = color::get_inverted_red(raw_pixel);

        color::set_red(raw_pixel, new_red);
    });
}

#[wasm_bindgen]
pub fn neue(img: &mut ImageWasm) {
    let chunked_raw_pixels = get_chunked_raw_pixels(img);

    chunked_raw_pixels.for_each(|raw_pixel| {
        let new_blue = color::get_inverted_blue(raw_pixel);

        color::set_blue(raw_pixel, new_blue);
    });
}

#[wasm_bindgen]
pub fn lix(img: &mut ImageWasm) {
    let chunked_raw_pixels = get_chunked_raw_pixels(img);

    chunked_raw_pixels.for_each(|raw_pixel| {
        let new_red = color::get_inverted_red(raw_pixel);
        let new_green = color::get_inverted_green(raw_pixel);

        color::set_red(raw_pixel, new_red);
        color::set_green(raw_pixel, new_green);
    });
}

#[wasm_bindgen]
pub fn ryo(img: &mut ImageWasm) {
    let chunked_raw_pixels = get_chunked_raw_pixels(img);

    chunked_raw_pixels.for_each(|raw_pixel| {
        let new_red = color::get_inverted_red(raw_pixel);
        let new_blue = color::get_inverted_blue(raw_pixel);

        color::set_red(raw_pixel, new_red);
        color::set_blue(raw_pixel, new_blue);
    });
}

fn channel_grayscale(img: &mut ImageWasm, color_idx: usize) {
    let chunked_raw_pixels = get_chunked_raw_pixels(img);

    chunked_raw_pixels.for_each(|raw_pixel| {
        let channel_value = raw_pixel[color_idx];

        color::set_red(raw_pixel, channel_value);
        color::set_green(raw_pixel, channel_value);
        color::set_blue(raw_pixel, channel_value);
    });
}

#[wasm_bindgen(js_name = redGrayscale)]
pub fn red_grayscale(img: &mut ImageWasm) {
    channel_grayscale(img, color::Rgba::RED);
}

#[wasm_bindgen(js_name = greenGrayscale)]
pub fn green_grayscale(img: &mut ImageWasm) {
    channel_grayscale(img, color::Rgba::GREEN);
}

#[wasm_bindgen(js_name = blueGrayscale)]
pub fn blue_grayscale(img: &mut ImageWasm) {
    channel_grayscale(img, color::Rgba::BLUE);
}

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
