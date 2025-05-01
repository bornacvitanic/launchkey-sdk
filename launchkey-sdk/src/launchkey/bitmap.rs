use image::DynamicImage;

/// Represents a 128x64 monochrome bitmap for Launchkey MK4 displays
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchkeyBitmap([u8; 1216]);

impl LaunchkeyBitmap {
    /// Creates a new bitmap from a 1216-byte array, where each byte encodes pixels in a specific format.
    pub fn new(data: [u8; 1216]) -> Self {
        Self(data)
    }

    /// Converts a full image to a Launchkey-compatible 128x64 monochrome bitmap using a brightness threshold.
    pub fn from_image(img: DynamicImage, threshold: u8) -> Result<Self, String> {
        const DISPLAY_WIDTH: u32 = 128;
        const DISPLAY_HEIGHT: u32 = 64;
        const BYTES_PER_ROW: u32 = 19;

        let img = img.resize_exact(DISPLAY_WIDTH, DISPLAY_HEIGHT, image::imageops::Nearest);
        let img = img.to_luma8();

        let mut data = [0u8; 1216];

        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let pixel = img.get_pixel(x, y)[0];
                if pixel > threshold {
                    let byte_index = ((y * BYTES_PER_ROW) + (x / 7)) as usize;
                    let bit_offset = 6 - (x % 7); // Leftmost pixel = highest bit (bit 7)
                    data[byte_index] |= 1 << bit_offset;
                }
            }
        }

        Ok(Self(data))
    }
}

impl TryFrom<&[u8]> for LaunchkeyBitmap {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 1216 {
            Err(format!("Invalid length: {} (must be 1216)", value.len()))
        } else {
            let mut buffer = [0u8; 1216];
            buffer.copy_from_slice(value);
            Ok(Self(buffer))
        }
    }
}

impl From<LaunchkeyBitmap> for [u8; 1216] {
    fn from(val: LaunchkeyBitmap) -> Self {
        val.0
    }
}

impl AsRef<[u8; 1216]> for LaunchkeyBitmap {
    fn as_ref(&self) -> &[u8; 1216] {
        &self.0
    }
}