#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegularButton {
    Shift = 0x3F,
    PreviousTrack = 0x67,
    NextTrack = 0x66,
    EncoderMoveUp = 0x33,
    EncoderMoveDown = 0x34,
    PadBankUp = 0x6A,
    PadBankDown = 0x6B,
    SceneLaunch = 0x68,
    Function = 0x69,
    CaptureMIDI = 0x4A,
    UndoRedo = 0x4D,
    Quantise = 0x4B,
    Metronome = 0x4C,
    Stop = 0x74,
    Loop = 0x76,
    Play = 0x73,
    Record = 0x75,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiniButton {
    Shift = 0x3F,
    Play = 0x73,
    Record = 0x75,
    EncoderMoveUp = 0x33,
    EncoderMoveDown = 0x34,
    PadBankUp = 0x6A,
    PadBankDown = 0x6B,
    SceneLaunch = 0x68,
    Function = 0x69,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchKeyButton {
    Regular(RegularButton),
    Mini(MiniButton),
}

impl LaunchKeyButton {
    /// Get the correct button index based on whether it's a Regular or Mini Launchkey
    pub fn to_index(self) -> u8 {
        match self {
            LaunchKeyButton::Regular(regular_button) => regular_button as u8,
            LaunchKeyButton::Mini(mini_button) => mini_button as u8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Brightness(u8);

impl Brightness {
    pub const MIN: u8 = 0x00;
    pub const MAX: u8 = 0x7F;

    /// Tries to create a new `Brightness`, returning `None` if out of range.
    pub fn new(value: u8) -> Option<Self> {
        if value <= Self::MAX {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns the minimum brightness (off).
    pub fn min() -> Self {
        Self(Self::MIN)
    }

    /// Returns the maximum brightness (full brightness).
    pub fn max() -> Self {
        Self(Self::MAX)
    }

    /// Returns the inner `u8` value.
    pub fn value(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for Brightness {
    type Error = &'static str;

    /// Brightness must be in the range 0–127 (0x00–0x7F in HEX).
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 127 {
            Ok(Brightness(value))
        } else {
            Err("Brightness must be in the range 0–127 (0x00–0x7F in HEX).")
        }
    }
}
