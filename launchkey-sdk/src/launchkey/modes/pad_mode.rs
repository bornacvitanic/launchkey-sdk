#[derive(Debug, Clone)]
pub enum PadMode {
    /// Used for controlling Digital Audio Workstations.
    DAW,

    /// Pads function as drum triggers in standalone (MIDI) mode.
    Drum,

    /// Activates a DAW-controlled drum mode,
    /// allowing the DAW to manage pad colors and receive MIDI messages
    /// through the DAW MIDI port. Selecting `Drum` will disable this mode
    /// and return to standalone drum operation.
    DrumDAW,

    /// Pads trigger custom chord mappings.
    UserChords,

    /// Chord Map mode: Pads trigger predefined chord progressions.
    ChordMap,

    /// User-defined pad mapping (slot 1).
    Custom1,

    /// User-defined pad mapping (slot 2).
    Custom2,

    /// User-defined pad mapping (slot 3).
    Custom3,

    /// User-defined pad mapping (slot 4).
    Custom4,

    /// Pads trigger arpeggiator patterns.
    ArpPattern,
}

impl PadMode {
    pub fn as_bytes(&self) -> Vec<u8> {
        let value = match self {
            PadMode::DAW => 0x02,
            PadMode::Drum => 0x01,
            PadMode::DrumDAW => 0x0F,
            PadMode::UserChords => 0x04,
            PadMode::ChordMap => 0x0E,
            PadMode::Custom1 => 0x05,
            PadMode::Custom2 => 0x06,
            PadMode::Custom3 => 0x07,
            PadMode::Custom4 => 0x08,
            PadMode::ArpPattern => 0x0D,
        };
        vec![0xB6, 0x1D, value]
    }
}