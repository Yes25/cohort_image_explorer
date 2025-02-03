use image::ImageEncoder;
use std::io::Write;

use std::collections::HashMap;

use base64::prelude::*;
use serde::Serialize;

use nifti_decoder::Header;

pub fn build_base64_image_vec(header: &Header, volume: Vec<u8>) -> ImageData {
    let size_x = header.dimensions[0] as u32;
    let size_y = header.dimensions[1] as u32;
    let size_z = header.dimensions[2] as u32;

    let mut slices = Vec::with_capacity(size_z as usize);

    for i in 0..(size_z - 1) {
        let start_idx = (i * size_x * size_y) as usize;
        let end_idx = ((i + 1) * size_x * size_y) as usize;

        let raw_pix_data = &volume[start_idx..end_idx];

        let color = image::ExtendedColorType::L8;
        let mut encoded_image = Vec::new();

        image::codecs::png::PngEncoder::new(encoded_image.by_ref())
            .write_image(raw_pix_data, size_x, size_y, color)
            .expect("error encoding pixels as PNG");

        let base64_png = BASE64_STANDARD.encode(encoded_image);
        slices.push(base64_png);
    }
    let metadata = HashMap::from([("dims".to_owned(), format!("[{size_x}, {size_y}, {size_z}]"))]);
    ImageData { metadata, slices }
}

#[derive(Serialize, Debug)]
pub struct ImageData {
    pub metadata: HashMap<String, String>,
    pub slices: Vec<String>,
}
