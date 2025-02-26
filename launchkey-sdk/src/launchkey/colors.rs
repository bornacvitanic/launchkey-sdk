#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorPaletteIndex(u8);

impl ColorPaletteIndex {
    /// Creates a new `PaletteIndex` with a value clamped to the range [0, 127].
    fn clamp_new(value: u8) -> Self {
        Self(value.min(127))
    }

    /// Creates a new `ColorPaletteIndex` if the value is within the valid range [0, 127].
    pub fn try_from(value: u8) -> Result<Self, String> {
        if value <= 127 {
            Ok(Self(value))
        } else {
            Err(format!("Invalid color index: {}. Must be between 0 and 127.", value))
        }
    }

    /// Converts the `ColorPaletteIndex` to its raw `u8` value.
    pub fn as_u8(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Tries to create a new `Color` if all RGB values are within the valid range [0, 127].
    pub fn try_new(r: u8, g: u8, b: u8) -> Result<Self, String> {
        if r <= 127 && g <= 127 && b <= 127 {
            Ok(Self { r, g, b })
        } else {
            Err(format!(
                "Invalid RGB value: R={}, G={}, B={}. All values must be between 0 and 127.",
                r, g, b
            ))
        }
    }

    /// Creates a new `Color` with clamped values within the valid range [0, 127].
    /// This is useful for internal use where invalid values might slip through.
    pub(crate) fn clamp_new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r.min(127),
            g: g.min(127),
            b: b.min(127),
        }
    }

    /// Creates a new `Color` by converting RGB values from the full range [0, 255] to [0, 127].
    /// Automatically scales down the values.
    pub fn from_full_range(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: (r as f32 * 127.0 / 255.0) as u8,
            g: (g as f32 * 127.0 / 255.0) as u8,
            b: (b as f32 * 127.0 / 255.0) as u8,
        }
    }

    /// Creates a new `Color` by converting RGB values from the color palette range [97, 255] to [0, 127].
    pub fn from_color_palette_range(r: u8, g: u8, b: u8) -> Self {
        Self::map_rgb(r, g, b, 97, 255)
    }

    /// Creates a new `Color` by converting RGB values from the specified input range to [0, 127].
    pub fn map_rgb(r: u8, g: u8, b: u8, in_min: u8, in_max: u8) -> Self {
        let out_min = 0;
        let out_max = 127;
        let map_value = |value: u8| -> u8 {
            ((value as f32 - in_min as f32) / (in_max as f32 - in_min as f32)
                * (out_max as f32 - out_min as f32) + out_min as f32) as u8
        };

        Self {
            r: map_value(r),
            g: map_value(g),
            b: map_value(b),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommonColor {
    Off,
    DarkGray,
    LightGray,
    White,
    DimRed,
    NormalRed,
    BrightRed,
    DimYellow,
    NormalYellow,
    BrightYellow,
    DimGreen,
    NormalGreen,
    BrightGreen,
    DimCyan,
    NormalCyan,
    BrightCyan,
    DimBlue,
    NormalBlue,
    BrightBlue,
    DimPurple,
    NormalPurple,
    BrightPurple,
    DimPink,
    NormalPink,
    BrightPink,
}

impl CommonColor {
    /// Converts the `CommonColor` to a `ColorPaletteIndex`.
    pub fn to_palette_index(self) -> ColorPaletteIndex {
        let index = match self {
            CommonColor::Off => 0x00,
            CommonColor::DarkGray => 0x01,
            CommonColor::LightGray => 0x02,
            CommonColor::White => 0x03,
            CommonColor::DimRed => 0x07,
            CommonColor::NormalRed => 0x06,
            CommonColor::BrightRed => 0x05,
            CommonColor::DimYellow => 0x0F,
            CommonColor::NormalYellow => 0x0E,
            CommonColor::BrightYellow => 0x0D,
            CommonColor::DimGreen => 0x17,
            CommonColor::NormalGreen => 0x16,
            CommonColor::BrightGreen => 0x15,
            CommonColor::DimCyan => 0x27,
            CommonColor::NormalCyan => 0x26,
            CommonColor::BrightCyan => 0x25,
            CommonColor::DimBlue => 0x2F,
            CommonColor::NormalBlue => 0x2E,
            CommonColor::BrightBlue => 0x2D,
            CommonColor::DimPurple => 0x33,
            CommonColor::NormalPurple => 0x32,
            CommonColor::BrightPurple => 0x31,
            CommonColor::DimPink => 0x37,
            CommonColor::NormalPink => 0x36,
            CommonColor::BrightPink => 0x35,
        };
        ColorPaletteIndex::clamp_new(index)
    }

    /// Converts the `CommonColor` to a `Color`.
    pub fn to_color(self) -> Color {
        match self {
            CommonColor::Off => Color::from_color_palette_range(97, 97, 97),
            CommonColor::DarkGray => Color::from_color_palette_range(179, 179, 179),
            CommonColor::LightGray => Color::from_color_palette_range(221, 221, 221),
            CommonColor::White => Color::from_color_palette_range(255, 255, 255),
            CommonColor::DimRed => Color::from_color_palette_range(179, 97, 97),
            CommonColor::NormalRed => Color::from_color_palette_range(221, 97, 97),
            CommonColor::BrightRed => Color::from_color_palette_range(255, 97, 97),
            CommonColor::DimYellow => Color::from_color_palette_range(179, 179, 97),
            CommonColor::NormalYellow => Color::from_color_palette_range(221, 221, 97),
            CommonColor::BrightYellow => Color::from_color_palette_range(255, 255, 97),
            CommonColor::DimGreen => Color::from_color_palette_range(97, 179, 97),
            CommonColor::NormalGreen => Color::from_color_palette_range(97, 221, 97),
            CommonColor::BrightGreen => Color::from_color_palette_range(97, 255, 97),
            CommonColor::DimCyan => Color::from_color_palette_range(97, 161, 179),
            CommonColor::NormalCyan => Color::from_color_palette_range(97, 199, 221),
            CommonColor::BrightCyan => Color::from_color_palette_range(97, 238, 255),
            CommonColor::DimBlue => Color::from_color_palette_range(97, 97, 179),
            CommonColor::NormalBlue => Color::from_color_palette_range(97, 97, 221),
            CommonColor::BrightBlue => Color::from_color_palette_range(97, 97, 255),
            CommonColor::DimPurple => Color::from_color_palette_range(118, 97, 179),
            CommonColor::NormalPurple => Color::from_color_palette_range(129, 97, 221),
            CommonColor::BrightPurple => Color::from_color_palette_range(161, 97, 255),
            CommonColor::DimPink => Color::from_color_palette_range(179, 97, 179),
            CommonColor::NormalPink => Color::from_color_palette_range(221, 97, 221),
            CommonColor::BrightPink => Color::from_color_palette_range(255, 97, 255),
        }
    }
}