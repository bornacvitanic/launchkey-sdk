use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Encoder {
    Encoder1,
    Encoder2,
    Encoder3,
    Encoder4,
    Encoder5,
    Encoder6,
    Encoder7,
    Encoder8,
}

impl Encoder {
    /// Get the Control Change index for the encoder in absolute mode
    pub fn to_absolute_mode_index(self) -> u8 {
        match self {
            Encoder::Encoder1 => 0x15,
            Encoder::Encoder2 => 0x16,
            Encoder::Encoder3 => 0x17,
            Encoder::Encoder4 => 0x18,
            Encoder::Encoder5 => 0x19,
            Encoder::Encoder6 => 0x1A,
            Encoder::Encoder7 => 0x1B,
            Encoder::Encoder8 => 0x1C,
        }
    }

    /// Get the Control Change index for the encoder in relative mode
    pub fn to_relative_mode_index(self) -> u8 {
        match self {
            Encoder::Encoder1 => 0x55,
            Encoder::Encoder2 => 0x56,
            Encoder::Encoder3 => 0x57,
            Encoder::Encoder4 => 0x58,
            Encoder::Encoder5 => 0x59,
            Encoder::Encoder6 => 0x5A,
            Encoder::Encoder7 => 0x5B,
            Encoder::Encoder8 => 0x5C,
        }
    }

    /// Get the correct Control Change index based on whether the encoder is in absolute or relative mode
    pub fn to_index(self, is_absolute: bool) -> u8 {
        if is_absolute {
            self.to_absolute_mode_index()
        } else {
            self.to_relative_mode_index()
        }
    }

    /// Get all possible variants of `Encoder`.
    pub fn all() -> impl Iterator<Item = Self> { Self::iter() }

    /// Get the count of all variants
    pub fn count() -> usize { Self::all().count() }

    /// Safely get a variant by its index
    pub fn from_index(index: usize) -> Option<Self> { Self::all().nth(index) }
}