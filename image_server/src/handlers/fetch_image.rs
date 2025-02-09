use dicom::core::Tag;
use dicom::dictionary_std::tags;
use dicom::object::from_reader;
use dicom::pixeldata::image::GenericImageView;
use dicom::pixeldata::PixelDecoder;

use tar::Archive;

use image::ImageEncoder;
use std::io::{Read, Write};

use std::cmp::Ordering;
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

// Dicom image handling
#[derive(Debug, Clone)]
pub struct DicomImageSlice {
    pub width: u32,
    pub height: u32,
    pub img_vec: Vec<u8>,
    location: Option<f32>,
}

fn decode_dicom_slice(dicom_file: Vec<u8>) -> DicomImageSlice {
    let dicom_obj = from_reader(dicom_file.as_slice()).unwrap();

    let location = match dicom_obj.element(Tag(0x0020, 0x1041)) {
        Ok(location) => Some(location.to_float32().unwrap()),
        Err(_e) => None,
    };

    let decoded_pixel_data = dicom_obj.decode_pixel_data().unwrap();
    let dyn_img = decoded_pixel_data.to_dynamic_image(0).unwrap();

    let (width, height) = dyn_img.dimensions();

    let luma_img = dyn_img.to_luma8();
    let img_vec = luma_img.to_vec();

    DicomImageSlice {
        width,
        height,
        img_vec,
        location,
    }
}

struct DicomSlices {
    slice: String,
    location: Option<f32>,
}

impl DicomSlices {}

pub fn generate_dicom_image(dicom_tar: Vec<u8>) -> ImageData {
    let mut archive = Archive::new(dicom_tar.as_slice());

    // TODO: Can I somehow get the number of files in the archive. Then create Vec::with_capacity() -> no alloc needed then
    let mut slices: Vec<DicomSlices> = Vec::new();

    let mut size_x = 0;
    let mut size_y = 0;

    for slice in archive.entries().unwrap() {
        let mut slice = slice.unwrap();

        let mut image_buf: Vec<u8> = Vec::new();
        slice.read_to_end(&mut image_buf).unwrap();
        let dcm_img = decode_dicom_slice(image_buf);

        if size_x == 0 {
            size_x = dcm_img.width;
            size_y = dcm_img.height;
        }

        let color = image::ExtendedColorType::L8;
        let mut encoded_image = Vec::new();

        image::codecs::png::PngEncoder::new(encoded_image.by_ref())
            .write_image(&dcm_img.img_vec, dcm_img.width, dcm_img.height, color)
            .expect("error encoding pixels as PNG");

        let base64_png = BASE64_STANDARD.encode(encoded_image);
        slices.push(DicomSlices {
            slice: base64_png,
            location: dcm_img.location,
        });
    }

    slices.sort_by(|a, b| {
        if let Some(a_location) = a.location {
            if let Some(b_location) = b.location {
                return b_location.total_cmp(&a_location);
            };
            return Ordering::Equal;
        };
        return Ordering::Equal;
    });

    let size_z = slices.len();
    let slices = slices.drain(..).map(|dcm_slice| dcm_slice.slice).collect();
    let metadata = HashMap::from([("dims".to_owned(), format!("[{size_x}, {size_y}, {size_z}]"))]);
    ImageData { metadata, slices }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_dicom_file_test() {
        let bytes = std::fs::read(
            "/Users/jesse/Downloads/02-09-2025-14-49-19_files_list/047MQ6EHD1JZ_HTRLYQ43T3A1.tar",
        )
        .unwrap();

        let mut archive = Archive::new(bytes.as_slice());

        for file in archive.entries().unwrap() {
            // Make sure there wasn't an I/O error
            let mut file = file.unwrap();

            // files implement the Read trait
            let mut image_buf: Vec<u8> = Vec::new();
            file.read_to_end(&mut image_buf).unwrap();
            decode_dicom_slice(image_buf);
        }
    }
}
