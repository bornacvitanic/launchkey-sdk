use crate::launchkey::surface::encoders::Encoder;
use crate::launchkey::surface::faders::Fader;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum DisplayTarget {
    Global(GlobalDisplayTarget),
    Contextual(ContextualDisplayTarget),
}

impl From<DisplayTarget> for u8 {
    fn from(target: DisplayTarget) -> Self {
        match target {
            DisplayTarget::Global(global) => global.into(),
            DisplayTarget::Contextual(contextual) => contextual.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GlobalDisplayTarget {
    Stationary,
    Temporary,
}

impl From<GlobalDisplayTarget> for DisplayTarget {
    fn from(target: GlobalDisplayTarget) -> Self {
        DisplayTarget::Global(target)
    }
}

impl From<GlobalDisplayTarget> for u8 {
    fn from(target: GlobalDisplayTarget) -> Self {
        match target {
            GlobalDisplayTarget::Stationary => 0x20,
            GlobalDisplayTarget::Temporary => 0x21,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ContextualDisplayTarget {
    Temporary(TemporaryTarget),
    ModeName(ModeNameTarget),
}

impl From<ContextualDisplayTarget> for DisplayTarget {
    fn from(target: ContextualDisplayTarget) -> Self {
        DisplayTarget::Contextual(target)
    }
}

impl From<ContextualDisplayTarget> for u8 {
    fn from(target: ContextualDisplayTarget) -> Self {
        match target {
            ContextualDisplayTarget::Temporary(temporary_target) => temporary_target.into(),
            ContextualDisplayTarget::ModeName(mode_name_target) => match mode_name_target {
                ModeNameTarget::Plugin => 0x25,
                ModeNameTarget::Mixer => 0x24,
                ModeNameTarget::Sends => 0x26,
                ModeNameTarget::Transport => 0x27,
                ModeNameTarget::DAW => 0x22,
                ModeNameTarget::Drum => 0x23,
                ModeNameTarget::Volume => 0x28,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TemporaryTarget {
    Encoder(Encoder),
    Fader(Fader),
}

impl From<TemporaryTarget> for u8 {
    fn from(temporary_target: TemporaryTarget) -> Self {
        match temporary_target {
            TemporaryTarget::Encoder(encoder) => encoder.to_index(true),
            TemporaryTarget::Fader(fader) => fader.to_index(),
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
    Volume,
}

impl ModeNameTarget {
    /// Get all possible variants of `ModeNameTarget`.
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

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum DisplayConfig {
    Cancel,
    Arrangement(Arrangement),
    Trigger,
}

#[derive(Debug, Clone)]
pub enum Arrangement {
    NameValue(String, String),              // (name, value)
    TitleNameValue(String, String, String), // (title, name, value)
    TitleEightNames(String, [String; 8]),   // (title, eight names)
    NameNumericValue(String),               // (name)
}

impl From<DisplayConfig> for u8 {
    fn from(config: DisplayConfig) -> Self {
        match config {
            DisplayConfig::Cancel => 0x00,
            DisplayConfig::Arrangement(arrangement) => match arrangement {
                Arrangement::NameValue(_, _) => 0x01,
                Arrangement::TitleNameValue(_, _, _) => 0x02,
                Arrangement::TitleEightNames(_, _) => 0x03,
                Arrangement::NameNumericValue(_) => 0x04,
            },
            DisplayConfig::Trigger => 0x7F,
        }
    }
}
