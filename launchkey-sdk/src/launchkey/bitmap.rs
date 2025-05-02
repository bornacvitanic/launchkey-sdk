use image::DynamicImage;

/// Width of the Launchkey MK4 display in pixels.
const DISPLAY_WIDTH: u32 = 128;

/// Height of the Launchkey MK4 display in pixels.
const DISPLAY_HEIGHT: u32 = 64;

/// Number of bytes per bitmap for the Launchkey MK4.
const BITMAP_SIZE: usize = 1216;

/// Number of bytes per row (128px / 7 bits per byte = ~18.285 ≈ 19).
const BYTES_PER_ROW: u32 = 19;

/// Represents a 128x64 monochrome bitmap for Launchkey MK4 displays
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchkeyBitmap([u8; BITMAP_SIZE]);

impl LaunchkeyBitmap {
    /// Creates a new bitmap from a 1216-byte array, where each byte encodes pixels in a specific format.
    pub fn new(data: [u8; BITMAP_SIZE]) -> Self {
        Self(data)
    }

    /// Converts a full image to a Launchkey-compatible 128x64 monochrome bitmap using a brightness threshold.
    pub fn from_image(img: DynamicImage, threshold: u8) -> Self {
        let img = img.resize_exact(DISPLAY_WIDTH, DISPLAY_HEIGHT, image::imageops::Nearest);
        let img = img.to_luma8();

        let mut data = [0u8; BITMAP_SIZE];

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

        Self(data)
    }
}

impl TryFrom<&[u8]> for LaunchkeyBitmap {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != BITMAP_SIZE {
            Err(format!("Invalid length: {} (must be {})", value.len(), BITMAP_SIZE))
        } else {
            let mut buffer = [0u8; BITMAP_SIZE];
            buffer.copy_from_slice(value);
            Ok(Self(buffer))
        }
    }
}

impl From<LaunchkeyBitmap> for [u8; BITMAP_SIZE] {
    fn from(val: LaunchkeyBitmap) -> Self {
        val.0
    }
}

impl AsRef<[u8; BITMAP_SIZE]> for LaunchkeyBitmap {
    fn as_ref(&self) -> &[u8; BITMAP_SIZE] {
        &self.0
    }
}