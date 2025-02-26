use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy)]
pub enum LEDMode {
    Stationary,
    Flashing,
    Pulsing,
}

impl LEDMode {
    pub(crate) fn to_midi_channel(self, pad_in_mode: PadInMode) -> u8 {
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