use wmidi::U7;

/// A wrapper type representing a valid index (0–127) into the Launchkey's color palette.
/// Used to assign predefined colors to pads and other elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorPaletteIndex(U7);

impl ColorPaletteIndex {
    /// Creates a new [`PaletteIndex`] with a value clamped to the range [0, 127].
    fn clamp_new(value: u8) -> Self {
        Self(U7::clamp_new(value))
    }

    /// Creates a new [`ColorPaletteIndex`] if the value is within the valid range [0, 127].
    pub fn try_from(value: u8) -> Result<Self, String> {
        U7::try_from(value).map(Self).map_err(|_| {
            format!(
                "Invalid color index: {}. Must be between 0 and 127.",
                value
            )
        })
    }

    /// Converts the [`ColorPaletteIndex`] to its raw `u8` value.
    pub fn as_u8(self) -> u8 {
        self.0.into()
    }
}

/// Represents an RGB color with component values limited to the range [0, 127].
/// Used for setting custom colors on the Launchkey device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: U7,
    pub g: U7,
    pub b: U7,
}

impl Color {
    /// Tries to create a new [`Color`] if all RGB values are within the valid range [0, 127].
    pub fn try_new(r: u8, g: u8, b: u8) -> Result<Self, String> {
        let r = U7::try_from(r).map_err(|_| format!("Invalid R value: {}. Must be between 0 and 127.", r))?;
        let g = U7::try_from(g).map_err(|_| format!("Invalid G value: {}. Must be between 0 and 127.", g))?;
        let b = U7::try_from(b).map_err(|_| format!("Invalid B value: {}. Must be between 0 and 127.", b))?;
        Ok(Self { r, g, b })
    }

    /// Creates a new [`Color`] with clamped values within the valid range [0, 127].
    /// This is useful for internal use where invalid values might slip through.
    pub(crate) fn clamp_new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: U7::clamp_new(r),
            g: U7::clamp_new(g),
            b: U7::clamp_new(b),
        }
    }

    /// Creates a new [`Color`] by converting RGB values from the full range [0, 255] to [0, 127].
    /// Automatically scales down the values.
    pub fn from_full_range(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: U7::from_u8_lossy((r as f32 * 127.0 / 255.0) as u8),
            g: U7::from_u8_lossy((g as f32 * 127.0 / 255.0) as u8),
            b: U7::from_u8_lossy((b as f32 * 127.0 / 255.0) as u8),
        }
    }

    /// Creates a new [`Color`] by converting RGB values from the color palette range [97, 255] to [0, 127].
    /// Ensures input values are at least 97, but doesn't clamp the upper bound.
    pub fn from_color_palette_range(r: u8, g: u8, b: u8) -> Self {
        Self::map_rgb(r.max(97), g.max(97), b.max(97), 97, 255)
    }

    /// Creates a new [`Color`] by converting RGB values from the specified input range to [0, 127].
    pub fn map_rgb(r: u8, g: u8, b: u8, in_min: u8, in_max: u8) -> Self {
        let out_min = 0;
        let out_max = 127;

        let map_value = |value: u8| -> U7 {
            let mapped = ((value as f32 - in_min as f32) / (in_max as f32 - in_min as f32)
                * (out_max as f32 - out_min as f32)
                + out_min as f32) as u8;
            U7::from_u8_lossy(mapped)
        };

        Self {
            r: map_value(r),
            g: map_value(g),
            b: map_value(b),
        }
    }
}

/// Enum representing a set of common named colors supported by the Launchkey palette.
/// These are mapped to fixed RGB values and palette indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommonColor {
    Off,
    DarkGray,
    LightGray,
    White,
    DimRed,
    NormalRed,
    BrightRed,
    DimOrange,
    NormalOrange,
    BrightOrange,
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
    /// Converts the [`CommonColor`] to a [`ColorPaletteIndex`].
    pub fn to_palette_index(self) -> ColorPaletteIndex {
        let index = match self {
            CommonColor::Off => 0x00,
            CommonColor::DarkGray => 0x01,
            CommonColor::LightGray => 0x02,
            CommonColor::White => 0x03,
            CommonColor::DimRed => 0x07,
            CommonColor::NormalRed => 0x06,
            CommonColor::BrightRed => 0x05,
            CommonColor::DimOrange => 0x0B,
            CommonColor::NormalOrange => 0x0A,
            CommonColor::BrightOrange => 0x09,
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

    /// Converts the [`CommonColor`] to a [`Color`].
    pub fn to_color(self) -> Color {
        match self {
            CommonColor::Off => Color::from_color_palette_range(97, 97, 97),
            CommonColor::DarkGray => Color::from_color_palette_range(179, 179, 179),
            CommonColor::LightGray => Color::from_color_palette_range(221, 221, 221),
            CommonColor::White => Color::from_color_palette_range(255, 255, 255),
            CommonColor::DimRed => Color::from_color_palette_range(179, 97, 97),
            CommonColor::NormalRed => Color::from_color_palette_range(221, 97, 97),
            CommonColor::BrightRed => Color::from_color_palette_range(255, 97, 97),
            CommonColor::DimOrange => Color::from_color_palette_range(179, 118, 97),
            CommonColor::NormalOrange => Color::from_color_palette_range(221, 140, 97),
            CommonColor::BrightOrange => Color::from_color_palette_range(255, 179, 97),
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

impl Into<ColorPaletteIndex> for CommonColor {
    fn into(self) -> ColorPaletteIndex {
        self.to_palette_index()
    }
}

impl Into<Color> for CommonColor {
    fn into(self) -> Color {
        self.to_color()
    }
}

// Trait to add the `clamp_new` functionality to U7
pub trait U7Ext {
    fn clamp_new(value: u8) -> U7;
}

impl U7Ext for U7 {
    fn clamp_new(value: u8) -> U7 {
        // Clamp the value to the range [0, 127]
        U7::from_u8_lossy(value.min(U7::MAX.into()))
    }
}