use crate::logic::color_type::ColorType;
use crate::logic::image_metadata::ImageMetadata;

use super::node::Node;
use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
    fs::File,
    io::{ErrorKind, Read, Write},
};

pub struct ImageCompressor {
    codes: HashMap<u8, Vec<bool>>,
    tree: Option<Node>,
}

impl ImageCompressor {
    pub fn new() -> Self {
        ImageCompressor {
            codes: HashMap::new(),
            tree: None,
        }
    }

    pub fn compress(
        &mut self,
        input_path: &str,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let img = image::open(input_path)?;
        let (w, h) = (img.width(), img.height());

        let (raw_px, color_type) = match img {
            image::DynamicImage::ImageLuma8(gray) => (gray.into_raw(), ColorType::Grayscale),
            image::DynamicImage::ImageRgb8(rgb) => (rgb.into_raw(), ColorType::Rgb),
            image::DynamicImage::ImageRgba8(rgba) => (rgba.into_raw(), ColorType::Rgba),
            _ => {
                let rgb = img.to_rgb8();
                (rgb.into_raw(), ColorType::Rgb)
            }
        };

        let metadata = ImageMetadata {
            width: w,
            height: h,
            color_type,
        };

        let transformed = self.apply_delta_encoding(&raw_px);

        self.build_tree(&transformed);
        self.generate_codes();

        let encoded_bits = self.encode(&transformed);
        let compressed_bytes = self.bits_to_bytes(&encoded_bits);

        self.write_image(
            output_path,
            &metadata,
            &raw_px,
            &compressed_bytes,
            encoded_bits.len(),
        )?;
        Ok(())
    }

    fn decompress(&mut self, input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
        let (metadata, original_data, compressed_bytes, bit_count) = self.read_image(input_path)?;

        self.build_tree(&original_data);
        self.generate_codes();

        let bits = self.bytes_to_bits(&compressed_bytes, bit_count);
        let decoded = self.decode(&bits);

        let px = self.reverse_delta_encoding(&decoded);

        let img = match metadata.color_type {
            ColorType::Grayscale => {
                let gray = image::GrayImage::from_raw(metadata.width, metadata.height, px)
                    .ok_or("Failed to create grayscale image")?;
                image::DynamicImage::ImageLuma8(gray)
            }
            ColorType::Rgb => {
                let rgb = image::RgbImage::from_raw(metadata.width, metadata.height, px)
                    .ok_or("Failed to create RGB image")?;
                image::DynamicImage::ImageRgb8(rgb)
            }
            ColorType::Rgba => {
                let rgba = image::RgbaImage::from_raw(metadata.width, metadata.height, px)
                    .ok_or("Failed to create RGBA image")?;
                image::DynamicImage::ImageRgba8(rgba)
            }
        };
        img.save(output_path)?;
        Ok(())
    }

    fn apply_delta_encoding(&self, data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(data.len());
        res.push(data[0]);

        for i in 1..data.len() {
            res.push(data[i].wrapping_sub(data[i - 1]));
        }

        res
    }

    fn reverse_delta_encoding(&self, data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(data.len());
        res.push(data[0]);

        for i in 1..data.len() {
            res.push(res[i - 1].wrapping_add(data[i]));
        }

        res
    }

    fn build_tree(&mut self, data: &[u8]) {
        let mut freq_map: HashMap<u8, usize> = HashMap::new();

        for &byte in data {
            *freq_map.entry(byte).or_insert(0) += 1;
        }

        if freq_map.is_empty() {
            return;
        }

        if freq_map.len() == 1 {
            let (&val, &freq) = freq_map.iter().next().unwrap();
            self.tree = Some(Node::new_leaf(freq, val));
            return;
        }

        let mut heap = BinaryHeap::new();

        for (&val, &freq) in &freq_map {
            heap.push(Node::new_leaf(freq, val));
        }

        while heap.len() > 1 {
            let left = heap.pop().unwrap();
            let right = heap.pop().unwrap();
            heap.push(Node::new_internal(left.freq + right.freq, left, right));
        }

        self.tree = heap.pop();
    }

    fn generate_codes(&mut self) {
        self.codes.clear();
        if let Some(ref tree) = self.tree {
            let mut code = Vec::new();
            Self::generate_codes_helper(tree, &mut code, &mut self.codes);
        }
    }

    fn generate_codes_helper(
        node: &Node,
        code: &mut Vec<bool>,
        codes: &mut HashMap<u8, Vec<bool>>,
    ) {
        if let Some(val) = node.value {
            if code.is_empty() {
                codes.insert(val, vec![false]);
            } else {
                codes.insert(val, code.clone());
            }
        } else {
            if let Some(ref left) = node.left {
                code.push(false);
                Self::generate_codes_helper(left, code, codes);
                code.pop();
            }
            if let Some(ref right) = node.right {
                code.push(true);
                Self::generate_codes_helper(right, code, codes);
                code.pop();
            }
        }
    }

    fn encode(&self, data: &[u8]) -> Vec<bool> {
        let mut res = Vec::new();
        for &byte in data {
            if let Some(code) = self.codes.get(&byte) {
                res.extend_from_slice(code);
            }
        }
        res
    }

    fn decode(&self, bits: &[bool]) -> Vec<u8> {
        let mut res = Vec::new();
        if let Some(ref tree) = self.tree {
            if tree.value.is_some() {
                let val = tree.value.unwrap();
                for _ in 0..bits.len() {
                    res.push(val);
                }
                return res;
            }

            let mut curr = tree;
            for &bit in bits {
                curr = if bit {
                    curr.right.as_ref().unwrap()
                } else {
                    curr.left.as_ref().unwrap()
                };

                if let Some(val) = curr.value {
                    res.push(val);
                    curr = tree;
                }
            }
        }
        res
    }

    fn bits_to_bytes(&self, bits: &[bool]) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut curr_byte = 0u8;
        let mut bit_count = 0;

        for &bit in bits {
            if bit {
                curr_byte |= 1 << (7 - bit_count);
            }
            bit_count += 1;

            if bit_count == 8 {
                bytes.push(curr_byte);
                curr_byte = 0;
                bit_count = 0;
            }
        }

        if bit_count > 0 {
            bytes.push(curr_byte);
        }
        bytes
    }

    fn bytes_to_bits(&self, bytes: &[u8], total_bits: usize) -> Vec<bool> {
        let mut bits = Vec::new();
        for &byte in bytes {
            for i in (0..8).rev() {
                if bits.len() >= total_bits {
                    break;
                }
                bits.push((byte >> i) & 1 == 1);
            }
        }
        bits.truncate(total_bits);
        bits
    }

    fn write_image(
        &self,
        path: &str,
        metadata: &ImageMetadata,
        original: &[u8],
        compressed: &[u8],
        bit_count: usize,
    ) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        file.write_all(b"HIMG")?;
        file.write_all(&metadata.width.to_le_bytes())?;
        file.write_all(&metadata.height.to_le_bytes())?;
        file.write_all(&[metadata.color_type as u8])?;
        file.write_all(&(original.len() as u64).to_le_bytes())?;
        file.write_all(&(bit_count as u64).to_le_bytes())?;

        let mut freq_map: HashMap<u8, u32> = HashMap::new();
        for &byte in original {
            *freq_map.entry(byte).or_insert(0) += 1;
        }

        file.write_all(&(freq_map.len() as u32).to_le_bytes())?;
        for (&byte, &freq) in &freq_map {
            file.write_all(&[byte])?;
            file.write_all(&freq.to_le_bytes())?;
        }

        file.write_all(compressed)?;
        Ok(())
    }

    fn read_image(
        &self,
        path: &str,
    ) -> Result<(ImageMetadata, Vec<u8>, Vec<u8>, usize), Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut buf = [0u8; 4];
        file.read_exact(&mut buf)?;

        if &buf != b"HIMG" {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::InvalidData,
                "Invalid header",
            )));
        }

        file.read_exact(&mut buf)?;
        let w = u32::from_le_bytes(buf);

        file.read_exact(&mut buf)?;
        let h = u32::from_le_bytes(buf);

        let mut color_buf = [0u8; 1];
        file.read_exact(&mut color_buf)?;

        let color_type = ColorType::from_u8(color_buf[0]).ok_or_else(|| {
            Box::new(std::io::Error::new(
                ErrorKind::InvalidData,
                "Invalid color type",
            ))
        })?;
        let metadata = ImageMetadata {
            width: w,
            height: h,
            color_type,
        };

        let mut size_buf = [0u8; 8];
        file.read_exact(&mut size_buf)?;
        let _ori_size = u64::from_le_bytes(size_buf) as usize;

        file.read_exact(&mut size_buf)?;
        let bit_count = u64::from_le_bytes(size_buf) as usize;

        let mut freq_count_buf = [0u8; 4];
        file.read_exact(&mut freq_count_buf)?;
        let freq_count = u32::from_le_bytes(freq_count_buf) as usize;

        let mut ori_data = Vec::new();
        for _ in 0..freq_count {
            let mut byte_buf = [0u8; 1];
            file.read_exact(&mut byte_buf)?;
            let byte = byte_buf[0];

            let mut freq_buf = [0u8; 4];
            file.read_exact(&mut freq_buf)?;
            let freq = u32::from_le_bytes(freq_buf) as usize;

            for _ in 0..freq {
                ori_data.push(byte);
            }
        }

        let mut compressed = Vec::new();
        file.read_to_end(&mut compressed)?;

        Ok((metadata, ori_data, compressed, bit_count))
    }
}
