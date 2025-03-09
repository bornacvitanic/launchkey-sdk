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

impl Fader {
    /// Get the correct Control Change index
    pub fn to_index(self) -> u8 {
        match self {
            Fader::Fader1 => 0x05,
            Fader::Fader2 => 0x06,
            Fader::Fader3 => 0x07,
            Fader::Fader4 => 0x08,
            Fader::Fader5 => 0x09,
            Fader::Fader6 => 0x0A,
            Fader::Fader7 => 0x0B,
            Fader::Fader8 => 0x0C,
            Fader::Fader9 => 0x0D,
        }
    }

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
