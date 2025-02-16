use dicom::core::Tag;
use dicom::object::{from_reader, FileDicomObject, InMemDicomObject};
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
    let metadata = HashMap::from([(
        String::from("image"),
        HashMap::from([(
            String::from("dims"),
            format!("[{size_x}, {size_y}, {size_z}]"),
        )]),
    )]);
    ImageData { metadata, slices }
}

#[derive(Serialize, Debug)]
pub struct ImageData {
    pub metadata: HashMap<String, HashMap<String, String>>,
    pub slices: Vec<String>,
}

// Dicom image handling
pub struct DicomImageSlice {
    pub width: u32,
    pub height: u32,
    pub img_vec: Vec<u8>,
    location: Option<f32>,
}

fn extract_dicom_header_info(
    dicom_obj: &FileDicomObject<InMemDicomObject>,
) -> HashMap<String, String> {
    let patient_name = match dicom_obj.element(Tag(0x0010, 0x0010)) {
        Ok(patient_name) => patient_name.to_str().unwrap().into_owned(),
        Err(_e) => "".to_owned(),
    };
    let patient_id = match dicom_obj.element(Tag(0x0010, 0x0020)) {
        Ok(patient_id) => patient_id.to_str().unwrap().into_owned(),
        Err(_e) => "".to_owned(),
    };
    let patient_dob = match dicom_obj.element(Tag(0x0010, 0x0030)) {
        Ok(patient_dob) => patient_dob.to_str().unwrap().into_owned(),
        Err(_e) => "".to_owned(),
    };
    let patient_sex = match dicom_obj.element(Tag(0x0010, 0x0040)) {
        Ok(patient_sex) => patient_sex.to_str().unwrap().into_owned(),
        Err(_e) => "".to_owned(),
    };
    HashMap::from([
        (String::from("name"), patient_name),
        (String::from("id"), patient_id),
        (String::from("birth_date"), patient_dob),
        (String::from("sex"), patient_sex),
    ])
}

fn decode_dicom_slice(
    dicom_file: Vec<u8>,
    first_slice: bool,
) -> (DicomImageSlice, Option<HashMap<String, String>>) {
    let dicom_obj = from_reader(dicom_file.as_slice()).unwrap();

    let mut header_info = None;
    if first_slice {
        header_info = Some(extract_dicom_header_info(&dicom_obj));
    }

    let location = match dicom_obj.element(Tag(0x0020, 0x1041)) {
        Ok(location) => Some(location.to_float32().unwrap()),
        Err(_e) => None,
    };

    let decoded_pixel_data = dicom_obj.decode_pixel_data().unwrap();
    let dyn_img = decoded_pixel_data.to_dynamic_image(0).unwrap();

    let (width, height) = dyn_img.dimensions();

    let luma_img = dyn_img.to_luma8();
    let img_vec = luma_img.to_vec();

    (
        DicomImageSlice {
            width,
            height,
            img_vec,
            location,
        },
        header_info,
    )
}

struct DicomBase64Slice {
    slice: String,
    location: Option<f32>,
}

pub fn generate_dicom_image(dicom_tar: Vec<u8>) -> ImageData {
    let mut archive = Archive::new(dicom_tar.as_slice());

    // TODO: Can I somehow get the number of files in the archive. Then create Vec::with_capacity() -> no alloc needed then
    let mut slices: Vec<DicomBase64Slice> = Vec::new();

    let mut size_x = 0;
    let mut size_y = 0;

    let mut metadata = HashMap::new();
    let mut is_first_slice = true;

    for slice in archive.entries().unwrap() {
        let mut slice = slice.unwrap();

        let mut image_buf: Vec<u8> = Vec::new();
        slice.read_to_end(&mut image_buf).unwrap();
        let (dcm_img, header_info) = decode_dicom_slice(image_buf, is_first_slice);

        if let Some(header_info) = header_info {
            metadata.insert(String::from("patient"), header_info);
            is_first_slice = false;
        }

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
        slices.push(DicomBase64Slice {
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
    metadata.insert(
        String::from("image"),
        HashMap::from([(
            String::from("dims"),
            format!("[{size_x}, {size_y}, {size_z}]"),
        )]),
    );
    ImageData { metadata, slices }
}
