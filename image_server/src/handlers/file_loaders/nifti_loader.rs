use nifti::{InMemNiftiVolume, NiftiObject, NiftiVolume, ReaderOptions};
use std::path::PathBuf;


fn load_nifti_file(path: PathBuf) -> Vec<ImageSlice> {
    let file_name = path.to_str().unwrap().to_owned();

    let obj = ReaderOptions::new().read_file(path).unwrap();
    let volume = obj.volume().to_owned();

    nifi_to_rgba(volume, file_name)
}

fn nifi_to_rgba(volume: InMemNiftiVolume, file_name: String) -> Vec<ImageSlice> {
    let dims = volume.dim();
    let height = dims[0] as usize;
    let width = dims[1] as usize;
    let num_slices = dims[2] as usize;

    let num_slice_pixels = height * width;

    // let raw_data = volume.into_raw_data();
    let raw_data = volume.raw_data();
    let (min, max) = get_min_max(raw_data);

    let mut img_vec: Vec<ImageSlice> = vec![];

    for slice_idx in 0..num_slices {
        let start_idx = slice_idx * num_slice_pixels * 2;
        let end_idx = start_idx + num_slice_pixels * 2;

        let mut slice = Vec::<u8>::with_capacity(num_slice_pixels * 4);

        for i in (start_idx..end_idx).step_by(2) {
            // pixel vals are i16 -> 2 bytes -> conat two bytes to get value
            let i16_bytes = [raw_data[i], raw_data[i + 1]];
            // here little endian, could also be big? (is stored in volume struct)
            let pixel_val = scale_pix_val(min, max, u16::from_le_bytes(i16_bytes));

            slice.push(pixel_val); // r
            slice.push(pixel_val); // g
            slice.push(pixel_val); // b
            slice.push(255); // hue
        }

        img_vec.push(ImageSlice {
            file_name: file_name.clone(),
            width: width as u32,
            height: height as u32,
            rgba_vec: slice,
            location: Some(slice_idx as f32),
        });
    }
    img_vec
}

fn get_min_max(data: &[u8]) -> (u16, u16) {
    let mut max: u16 = u16::MIN;
    let mut min: u16 = u16::MAX;
    for i in (0..data.len()).step_by(2) {
        let val = u16::from_le_bytes([data[i], data[i + 1]]);
        if val > max {
            max = val;
        }
        if val < min {
            min = val;
        }
    }
    (min, max)
}

fn scale_pix_val(min: u16, max: u16, pix_val: u16) -> u8 {
    (((pix_val - min) as f32 / max as f32) * 255.) as u8
}
