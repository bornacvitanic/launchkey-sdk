use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum DisplayTarget {
    Temporary(u8), // 00h - 1Fh
    Stationary,
    GlobalTemporary,
    ModeName(ModeNameTarget),
}

impl From<DisplayTarget> for u8 {
    fn from(target: DisplayTarget) -> Self {
        match target {
            DisplayTarget::Temporary(idx) => idx,
            DisplayTarget::Stationary => 0x20,
            DisplayTarget::GlobalTemporary => 0x21,
            DisplayTarget::ModeName(mode_name_target) => {
                match mode_name_target {
                    ModeNameTarget::Plugin => 0x25,
                    ModeNameTarget::Mixer => 0x24,
                    ModeNameTarget::Sends => 0x26,
                    ModeNameTarget::Transport => 0x27,
                    ModeNameTarget::DAW => 0x22,
                    ModeNameTarget::Drum => 0x23,
                    ModeNameTarget::Volume => 0x28,
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display)]
pub enum ModeNameTarget {
    Plugin,
    Mixer,
    Sends,
    Transport,
    DAW,
    Drum,
    Volume
}

impl ModeNameTarget {
    /// Get all possible variants of `ModeNameTarget`.
    pub fn all() -> impl Iterator<Item = Self> { Self::iter() }

    /// Get the count of all variants
    pub fn count() -> usize { Self::all().count() }

    /// Safely get a variant by its index
    pub fn from_index(index: usize) -> Option<Self> { Self::all().nth(index) }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum DisplayConfig {
    Cancel,
    Arrangement(Arrangement),
    Trigger,
}

#[derive(Debug, Clone, Copy)]
pub enum Arrangement {
    NameValue,
    TitleNameValue,
    TitleEightNames,
    NameNumericValue,
}

impl From<DisplayConfig> for u8 {
    fn from(config: DisplayConfig) -> Self {
        match config {
            DisplayConfig::Cancel => 0x00,
            DisplayConfig::Arrangement(arrangement) => {
                match arrangement {
                    Arrangement::NameValue => 0x01,
                    Arrangement::TitleNameValue => 0x02,
                    Arrangement::TitleEightNames => 0x03,
                    Arrangement::NameNumericValue => 0x04,
                }
            },
            DisplayConfig::Trigger => 0x7F,
        }
    }
}