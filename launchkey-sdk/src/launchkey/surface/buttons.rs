use std::ops::Deref;
use crate::bidirectional_enum_mappings;
use wmidi::ControlValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchKeyButton {
    Shift,
    PreviousTrack,
    NextTrack,
    EncoderMoveUp,
    EncoderMoveDown,
    PadBankUp,
    PadBankDown,
    SceneLaunch,
    Function,
    CaptureMIDI,
    UndoRedo,
    Quantise,
    Metronome,
    Stop,
    Loop,
    Play,
    Record,
}

bidirectional_enum_mappings!(LaunchKeyButton, u8, {
    Shift => 0x3F,
    PreviousTrack => 0x67,
    NextTrack => 0x66,
    EncoderMoveUp => 0x33,
    EncoderMoveDown => 0x34,
    PadBankUp => 0x6A,
    PadBankDown => 0x6B,
    SceneLaunch => 0x68,
    Function => 0x69,
    CaptureMIDI => 0x4A,
    UndoRedo => 0x4D,
    Quantise => 0x4B,
    Metronome => 0x4C,
    Stop => 0x74,
    Loop => 0x76,
    Play => 0x73,
    Record => 0x75,
});

// Wrapper for Mini LaunchKey buttons, containing only the subset of buttons
pub struct MiniLaunchKeyButton(pub LaunchKeyButton);

impl MiniLaunchKeyButton {
    pub const SUPPORTED: [LaunchKeyButton; 9] = [
        LaunchKeyButton::Shift,
        LaunchKeyButton::Play,
        LaunchKeyButton::Record,
        LaunchKeyButton::EncoderMoveUp,
        LaunchKeyButton::EncoderMoveDown,
        LaunchKeyButton::PadBankUp,
        LaunchKeyButton::PadBankDown,
        LaunchKeyButton::SceneLaunch,
        LaunchKeyButton::Function,
    ];

    pub fn from_launchkey_button(button: LaunchKeyButton) -> Option<Self> {
        if Self::SUPPORTED.contains(&button) {
            Some(MiniLaunchKeyButton(button))
        } else {
            None
        }
    }
}

impl Deref for MiniLaunchKeyButton {
    type Target = LaunchKeyButton;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Wrapper for Regular LaunchKey buttons, containing all buttons
pub struct RegularLaunchKeyButton(pub LaunchKeyButton);

impl RegularLaunchKeyButton {
    pub const SUPPORTED: [LaunchKeyButton; 17] = [
        LaunchKeyButton::Shift,
        LaunchKeyButton::PreviousTrack,
        LaunchKeyButton::NextTrack,
        LaunchKeyButton::EncoderMoveUp,
        LaunchKeyButton::EncoderMoveDown,
        LaunchKeyButton::PadBankUp,
        LaunchKeyButton::PadBankDown,
        LaunchKeyButton::SceneLaunch,
        LaunchKeyButton::Function,
        LaunchKeyButton::CaptureMIDI,
        LaunchKeyButton::UndoRedo,
        LaunchKeyButton::Quantise,
        LaunchKeyButton::Metronome,
        LaunchKeyButton::Stop,
        LaunchKeyButton::Loop,
        LaunchKeyButton::Play,
        LaunchKeyButton::Record,
    ];

    pub fn from_launchkey_button(button: LaunchKeyButton) -> Option<Self> {
        if Self::SUPPORTED.contains(&button) {
            Some(RegularLaunchKeyButton(button))
        } else {
            None
        }
    }
}

impl Deref for RegularLaunchKeyButton {
    type Target = LaunchKeyButton;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Pressed,
    Released,
}

impl TryFrom<ControlValue> for ButtonState {
    type Error = ();

    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        match value.into() {
            127 => Ok(ButtonState::Pressed),
            0 => Ok(ButtonState::Released),
            _ => Err(()),
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