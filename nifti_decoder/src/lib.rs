use flate2::read::GzDecoder;
use std::io::prelude::*;

pub fn decode_nifti(nifit_data: Vec<u8>) -> (Header, Vec<u8>) {
    let unpacked = assure_unpacked(nifit_data);
    let header = Header::from_bytes(&unpacked);

    assert_eq!(header.sizeof_hdr, 348);
    let volume = decode_volume(&unpacked, header.vox_offset, header.datatype, header.bitpix);

    (header, volume)
}

fn assure_unpacked(bytes: Vec<u8>) -> Vec<u8> {
    let header_size = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    if header_size != 348 {
        let unpacked = unpack_gz(bytes);
        let header_size = u32::from_le_bytes([unpacked[0], unpacked[1], unpacked[2], unpacked[3]]);
        assert_eq!(header_size, 348);
        return unpacked;
    } else {
        return bytes;
    }
}

fn unpack_gz(bytes: Vec<u8>) -> Vec<u8> {
    let byte_arr: &[u8] = &bytes;

    let mut gz_decoder = GzDecoder::new(byte_arr);
    let mut buf: Vec<u8> = Vec::new();
    gz_decoder.read_to_end(&mut buf).unwrap();
    buf
}

#[derive(Debug)]
pub struct Header {
    pub sizeof_hdr: u32,
    pub dim_info: u8,
    pub dimensions: Vec<u16>,
    pub datatype: u16,
    pub bitpix: u16,
    pub vox_offset: u32,
}

impl Header {
    fn from_bytes(bytes: &Vec<u8>) -> Self {
        let sizeof_hdr = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let dim_info = u8::from_le_bytes([bytes[39]]);
        let dimensions = create_dim_arr(bytes);
        let datatype = u16::from_le_bytes([bytes[70], bytes[71]]);
        let bitpix = u16::from_le_bytes([bytes[72], bytes[73]]);
        let vox_offset =
            f32::from_le_bytes([bytes[108], bytes[109], bytes[110], bytes[111]]) as u32;

        Header {
            sizeof_hdr,
            dim_info,
            dimensions,
            datatype,
            bitpix,
            vox_offset,
        }
    }
}

fn create_dim_arr(nifit_bytes: &Vec<u8>) -> Vec<u16> {
    let num_dims = u16::from_le_bytes([nifit_bytes[40], nifit_bytes[41]]);
    assert!(
        { num_dims >= 1 && num_dims <= 7 },
        "Num dims is not in range 1-7. Data might be bigendian. Bigendian is not implemented"
    );

    let mut dimensions: Vec<u16> = Vec::with_capacity(num_dims.into());
    let mut offset = 42;
    for _ in 0..num_dims {
        let dim_size = u16::from_le_bytes([nifit_bytes[offset], nifit_bytes[offset + 1]]);
        dimensions.push(dim_size);
        offset += 2;
    }
    dimensions
}

fn decode_volume(nifit_data: &Vec<u8>, vox_offset: u32, _datatype: u16, bitpix: u16) -> Vec<u8> {
    assert_eq!(
        bitpix, 16,
        "bitpix is not 16 -> other formats are not implemented, jet."
    );

    let vox_offset = vox_offset as usize;
    let (min, max) = get_min_max_u16(&nifit_data[vox_offset..]);
    let size_volume_bytes = nifit_data.len() - vox_offset;
    let size_pix_bytes = (bitpix / 8) as usize;
    let mut img_vec: Vec<u8> = Vec::with_capacity(size_volume_bytes / size_pix_bytes);

    for i in (0..nifit_data.len() - vox_offset).step_by(size_pix_bytes) {
        // pixel vals are i16 -> 2 bytes -> conat two bytes to get value
        let i16_bytes = [nifit_data[i + vox_offset], nifit_data[i + vox_offset + 1]];
        // here little endian, could also be big? (is stored in volume struct)
        let pixel_val = scale_pix_val(min, max, u16::from_le_bytes(i16_bytes));
        img_vec.push(pixel_val);
    }
    img_vec
}

fn get_min_max_u16(data: &[u8]) -> (u16, u16) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use image;
    use std::path::Path;

    #[test]
    fn unpack_gz_test() {
        let bytes = std::fs::read("/Users/jesse/Data/IXI-T1/IXI002-Guys-0828-T1.nii.gz").unwrap();
        let unpacked = unpack_gz(bytes);
        let header_size = u32::from_le_bytes([unpacked[0], unpacked[1], unpacked[2], unpacked[3]]);
        assert_eq!(header_size, 348)
    }

    #[test]
    fn decode_nifti_test() {
        let bytes = std::fs::read("/Users/jesse/Data/IXI-T1/IXI002-Guys-0828-T1.nii.gz").unwrap();
        let (header, volume) = decode_nifti(bytes);

        let size_x = header.dimensions[0] as u32;
        let size_y = header.dimensions[1] as u32;
        let size_z = header.dimensions[2] as u32;

        let img_theoretical_size: u32 = size_x * size_y * size_z;
        let img_acutal_size = volume.len() as u32;

        assert_eq!(img_theoretical_size, img_acutal_size);

        let start_idx = (75 * size_x * size_y) as usize;
        let end_idx = (76 * size_x * size_y) as usize;

        let buffer = &volume[start_idx..end_idx];
        let color = image::ExtendedColorType::L8;
        image::save_buffer(
            &Path::new("/Users/jesse/Desktop/test_image.png"),
            buffer,
            size_x,
            size_y,
            color,
        )
        .unwrap();
    }
}
