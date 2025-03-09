use crate::bidirectional_enum_mappings;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Fader {
    Fader1,
    Fader2,
    Fader3,
    Fader4,
    Fader5,
    Fader6,
    Fader7,
    Fader8,
    Fader9,
}

bidirectional_enum_mappings!(Fader, u8, {
    Fader1 => 0x05,
    Fader2 => 0x06,
    Fader3 => 0x07,
    Fader4 => 0x08,
    Fader5 => 0x09,
    Fader6 => 0x0A,
    Fader7 => 0x0B,
    Fader8 => 0x0C,
    Fader9 => 0x0D,
});

impl Fader {
    /// Get all possible variants of `Fader`.
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
