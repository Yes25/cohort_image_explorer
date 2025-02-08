use dicom::core::Tag;
use dicom::dictionary_std::tags;
use dicom::object::from_reader;
use dicom::pixeldata::image::GenericImageView;
use dicom::pixeldata::PixelDecoder;

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

// Dicom image handling
#[derive(Debug, Clone)]
pub struct DicomImageSlice {
    pub width: u32,
    pub height: u32,
    pub rgba_vec: Vec<u8>,
    location: Option<f32>,
}

fn decode_dicom_file(dicom_file: Vec<u8>) {
    let dicom_obj = from_reader(dicom_file.as_slice()).unwrap();

    let location = match dicom_obj.element(Tag(0x0020, 0x1041)) {
        Ok(location) => Some(location.to_float32().unwrap()),
        Err(_e) => None,
    };
    println!("Location: {}", location.unwrap());

    // let decoded_pixel_data = dicom_obj.decode_pixel_data().unwrap();
    // let dyn_img = decoded_pixel_data.to_dynamic_image(0).unwrap();

    // let (width, height) = dyn_img.dimensions();

    // let rgba_img = dyn_img.to_rgba8();
    // let rgba_vec = rgba_img.to_vec();

    // let image = DicomImageSlice {
    //     width,
    //     height,
    //     rgba_vec,
    //     location,
    // };
    // for i in 0..1000 {
    //     print!("{}", image.rgba_vec[i]);
    // }
    // println!("{}", image.width);
    // println!("{}", image.width);

    let patient_name = dicom_obj
        .element(tags::PATIENT_NAME)
        .unwrap()
        .to_str()
        .unwrap();
    let modality = dicom_obj
        .element_by_name("Modality")
        .unwrap()
        .to_str()
        .unwrap();

    println!("Patien Name: {}", patient_name);
    println!("Modality: {}", modality);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_dicom_file_test() {
        let bytes =
            std::fs::read("/Users/jesse/Downloads/dicom-test-files-master/data/WG04/J2KI/CT1_J2KI")
                .unwrap();
        decode_dicom_file(bytes)
    }
}
