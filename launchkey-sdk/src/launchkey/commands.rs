use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::launchkey::modes::pad_mode::PadMode;
use crate::launchkey::modes::fader_mode::FaderMode;
use crate::launchkey::modes::encoder_mode::EncoderMode;

const ENABLE_DAW_MODE: [u8; 3] = [0x9F, 0x0C, 0x7F];
const DISABLE_DAW_MODE: [u8; 3] = [0x9F, 0x0C, 0x00];
const SYSEX_TERMINATOR: u8 = 0xF7;

pub enum LaunchKeyCommand {
    EnableDAWMode,
    DisableDAWMode,
    SetPadMode(PadMode),
    SetEncoderMode(EncoderMode),
    SetFaderMode(FaderMode),
    SetButtonBrightness {
        launch_key_button: LaunchKeyButton,
        brightness: Brightness,
    },
    SetPadColor {
        pad_in_mode: PadInMode,
        mode: LEDMode,
        color: Color,
    },
    SetPadCustomColor {
        pad_in_mode: PadInMode,
        r: u8,
        g: u8,
        b: u8,
    },
    ConfigureDisplay {
        target: DisplayTarget,
        config: DisplayConfig,
    },
    SetScreenText {
        target: DisplayTarget,
        field: u8,
        text: String,
    },
    SendScreenBitmap {
        target: DisplayTarget,
        bitmap_data: [u8; 1216],
    },
}

impl LaunchKeyCommand {
    pub(crate) fn as_bytes(&self, sku: &LaunchKeySku) -> Vec<u8> {
        let header = sku.sys_ex_header();
        match self {
            LaunchKeyCommand::EnableDAWMode => ENABLE_DAW_MODE.to_vec(),
            LaunchKeyCommand::DisableDAWMode => DISABLE_DAW_MODE.to_vec(),
            LaunchKeyCommand::SetPadMode(mode) => mode.as_bytes(),
            LaunchKeyCommand::SetEncoderMode(mode) => mode.as_bytes(),
            LaunchKeyCommand::SetFaderMode(mode) => mode.as_bytes(),
            LaunchKeyCommand::SetButtonBrightness { launch_key_button, brightness } => {
                vec![0xB3, (*launch_key_button).to_index(), brightness.value()]
            },
            // Commands for pad LEDs
            LaunchKeyCommand::SetPadColor {
                pad_in_mode,
                mode,
                color,
            } => {
                let channel = mode.to_midi_channel(*pad_in_mode);
                vec![channel, (*pad_in_mode).to_index(), (*color) as u8]
            }
            LaunchKeyCommand::SetPadCustomColor {
                pad_in_mode,
                r,
                g,
                b,
            } => {
                let mut data = header.to_vec();
                data.extend_from_slice(&[0x01, 0x43, (*pad_in_mode).to_index(), *r, *g, *b]);
                data.push(SYSEX_TERMINATOR);
                data
            }

            // Commands for screen control
            LaunchKeyCommand::ConfigureDisplay { target, config } => {
                let mut data = header.to_vec();
                data.extend_from_slice(&[0x04, (*target).into(), (*config).into()]);
                data.push(SYSEX_TERMINATOR);
                data
            }
            LaunchKeyCommand::SetScreenText {
                target,
                field,
                text,
            } => {
                let mut data = header.to_vec();
                data.extend_from_slice(&[0x06, (*target).into(), *field]);
                data.extend(text.as_bytes());
                data.push(SYSEX_TERMINATOR);
                data
            }
            LaunchKeyCommand::SendScreenBitmap {
                target,
                bitmap_data,
            } => {
                let mut data = header.to_vec();
                data.extend_from_slice(&[0x09, (*target).into()]);
                data.extend_from_slice(bitmap_data);
                data.push(SYSEX_TERMINATOR);
                data
            }
        }
    }
}

#[derive(Debug)]
pub enum LaunchKeySku {
    Regular,
    Mini,
}

impl LaunchKeySku {
    pub fn sys_ex_header(&self) -> [u8; 6] {
        match self {
            LaunchKeySku::Regular => [0xF0, 0x00, 0x20, 0x29, 0x02, 0x14],
            LaunchKeySku::Mini => [0xF0, 0x00, 0x20, 0x29, 0x02, 0x13],
        }
    }
}

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

#[derive(Debug, Clone, Copy)]
pub enum LEDMode {
    Stationary,
    Flashing,
    Pulsing,
}

impl LEDMode {
    fn to_midi_channel(self, pad_in_mode: PadInMode) -> u8 {
        let base_channel = match self {
            LEDMode::Stationary => 0x90, // Channel 1
            LEDMode::Flashing => 0x91,   // Channel 2
            LEDMode::Pulsing => 0x92,    // Channel 3
        };
        match pad_in_mode {
            PadInMode::DAW(_) => base_channel, // DAW Pads always use default channels
            PadInMode::Drum(_) => base_channel + 9, // Offset for Drum pads in DAW mode
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Off = 0x00,
    LowRed = 0x07,
    MediumRed = 0x06,
    HighRed = 0x05,
    LowYellow = 0x0F,
    MediumYellow = 0x0E,
    HighYellow = 0x0D,
    LowGreen = 0x17,
    MediumGreen = 0x16,
    HighGreen = 0x15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Pad {
    PlugIn,
    Mixer,
    Sends,
    Transport,
    EncoderCustom1,
    EncoderCustom2,
    EncoderCustom3,
    EncoderCustom4,
    DAW,
    Drum,
    UserChord,
    ChordMap,
    PadCustom1,
    PadCustom2,
    PadCustom3,
    PadCustom4,
}

impl Pad {
    /// Get the MIDI index for the pad in DAW Mode
    pub fn to_daw_index(self) -> u8 {
        match self {
            Pad::PlugIn => 0x60,
            Pad::Mixer => 0x61,
            Pad::Sends => 0x62,
            Pad::Transport => 0x63,
            Pad::EncoderCustom1 => 0x64,
            Pad::EncoderCustom2 => 0x65,
            Pad::EncoderCustom3 => 0x66,
            Pad::EncoderCustom4 => 0x67,
            Pad::DAW => 0x70,
            Pad::Drum => 0x71,
            Pad::UserChord => 0x72,
            Pad::ChordMap => 0x73,
            Pad::PadCustom1 => 0x74,
            Pad::PadCustom2 => 0x75,
            Pad::PadCustom3 => 0x76,
            Pad::PadCustom4 => 0x77,
        }
    }

    /// Get the MIDI index for the pad in Drum Mode
    pub fn to_drum_index(self) -> u8 {
        match self {
            Pad::PlugIn => 0x28,
            Pad::Mixer => 0x29,
            Pad::Sends => 0x2A,
            Pad::Transport => 0x2B,
            Pad::EncoderCustom1 => 0x30,
            Pad::EncoderCustom2 => 0x31,
            Pad::EncoderCustom3 => 0x32,
            Pad::EncoderCustom4 => 0x33,
            Pad::DAW => 0x24,
            Pad::Drum => 0x25,
            Pad::UserChord => 0x26,
            Pad::ChordMap => 0x27,
            Pad::PadCustom1 => 0x2C,
            Pad::PadCustom2 => 0x2D,
            Pad::PadCustom3 => 0x2E,
            Pad::PadCustom4 => 0x7F,
        }
    }

    /// Get the correct MIDI index based on whether the pad is in DAW or Drum mode
    pub fn to_index(self, is_drum: bool) -> u8 {
        if is_drum {
            self.to_drum_index()
        } else {
            self.to_daw_index()
        }
    }
}

impl Pad {
    /// Get all possible variants of `Pad`.
    pub fn all() -> impl Iterator<Item = Self> {
        Self::iter()
    }

    /// Get the count of all variants.
    pub fn count() -> usize {
        Self::all().count()
    }

    /// Safely get a variant by its index.
    pub fn from_index(index: usize) -> Option<Self> {
        Self::all().nth(index)
    }
}

/// This enum ensures a Pad is explicitly either from DAW mode or Drum mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PadInMode {
    DAW(Pad),
    Drum(Pad),
}

impl PadInMode {
    /// Get the correct MIDI index based on whether it's in DAW or Drum mode
    pub fn to_index(self) -> u8 {
        match self {
            PadInMode::DAW(pad) => pad.to_daw_index(),
            PadInMode::Drum(pad) => pad.to_drum_index(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum DisplayTarget {
    Temporary(u8), // 00h - 1Fh
    Stationary,
    GlobalTemporary,
    DAWPadModeName,
    DAWDrumPadModeName,
    MixerEncoderModeName,
    PluginEncoderModeName,
    SendsEncoderModeName,
    TransportEncoderModeName,
    VolumeFaderModeName,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum DisplayConfig {
    Cancel,
    Trigger,
    Arrangement(u8),
}

impl From<DisplayTarget> for u8 {
    fn from(target: DisplayTarget) -> Self {
        match target {
            DisplayTarget::Temporary(idx) => idx,
            DisplayTarget::Stationary => 0x20,
            DisplayTarget::GlobalTemporary => 0x21,
            DisplayTarget::DAWPadModeName => 0x22,
            DisplayTarget::DAWDrumPadModeName => 0x23,
            DisplayTarget::MixerEncoderModeName => 0x24,
            DisplayTarget::PluginEncoderModeName => 0x25,
            DisplayTarget::SendsEncoderModeName => 0x26,
            DisplayTarget::TransportEncoderModeName => 0x27,
            DisplayTarget::VolumeFaderModeName => 0x28,
        }
    }
}

impl From<DisplayConfig> for u8 {
    fn from(config: DisplayConfig) -> Self {
        match config {
            DisplayConfig::Cancel => 0x00,
            DisplayConfig::Trigger => 0x7F,
            DisplayConfig::Arrangement(arrangement) => arrangement,
        }
    }
}