use crate::bidirectional_enum_mappings_with_mode;
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

bidirectional_enum_mappings_with_mode!(
    Pad, u8, PadCCIndex,
    {
        DAW => {
            PlugIn => 0x60,
            Mixer => 0x61,
            Sends => 0x62,
            Transport => 0x63,
            EncoderCustom1 => 0x64,
            EncoderCustom2 => 0x65,
            EncoderCustom3 => 0x66,
            EncoderCustom4 => 0x67,
            DAW => 0x70,
            Drum => 0x71,
            UserChord => 0x72,
            ChordMap => 0x73,
            PadCustom1 => 0x74,
            PadCustom2 => 0x75,
            PadCustom3 => 0x76,
            PadCustom4 => 0x77
        },
        Drum => {
            PlugIn => 0x28,
            Mixer => 0x29,
            Sends => 0x2A,
            Transport => 0x2B,
            EncoderCustom1 => 0x30,
            EncoderCustom2 => 0x31,
            EncoderCustom3 => 0x32,
            EncoderCustom4 => 0x33,
            DAW => 0x24,
            Drum => 0x25,
            UserChord => 0x26,
            ChordMap => 0x27,
            PadCustom1 => 0x2C,
            PadCustom2 => 0x2D,
            PadCustom3 => 0x2E,
            PadCustom4 => 0x2F
        }
    }
);

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
            PadInMode::DAW(pad) => pad.to_value_mode(PadCCIndex::DAW),
            PadInMode::Drum(pad) => pad.to_value_mode(PadCCIndex::Drum),
        }
    }
}