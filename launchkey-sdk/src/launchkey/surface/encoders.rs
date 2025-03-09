use crate::bidirectional_enum_mappings_with_mode;
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

bidirectional_enum_mappings_with_mode!(Encoder, u8, EncoderCCIndex,
    {
        Absolute => {
            Encoder1 => 0x15,
            Encoder2 => 0x16,
            Encoder3 => 0x17,
            Encoder4 => 0x18,
            Encoder5 => 0x19,
            Encoder6 => 0x1A,
            Encoder7 => 0x1B,
            Encoder8 => 0x1C,
        },
        Relative => {
            Encoder1 => 0x55,
            Encoder2 => 0x56,
            Encoder3 => 0x57,
            Encoder4 => 0x58,
            Encoder5 => 0x59,
            Encoder6 => 0x5A,
            Encoder7 => 0x5B,
            Encoder8 => 0x5C,
        }
    }
);

impl Encoder {
    /// Get the correct Control Change index based on whether the encoder is in absolute or relative mode
    pub fn to_index(self, is_absolute: bool) -> u8 {
        if is_absolute {
            self.to_value_mode(EncoderCCIndex::Absolute)
        } else {
            self.to_value_mode(EncoderCCIndex::Relative)
        }
    }

    /// Get all possible variants of `Encoder`.
    pub fn all() -> impl Iterator<Item = Self> {
        Self::iter()
    }

    /// Get the count of all variants
    pub fn count() -> usize {
        Self::all().count()
    }

    /// Safely get a variant by its index
    pub fn from_index(index: usize) -> Option<Self> {
        Self::all().nth(index)
    }
}
