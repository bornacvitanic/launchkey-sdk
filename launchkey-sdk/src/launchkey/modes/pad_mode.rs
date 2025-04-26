use crate::bidirectional_enum_mappings;

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

bidirectional_enum_mappings!(PadMode, u8, {
    DAW => 0x02,
    Drum => 0x01,
    DrumDAW => 0x0F,
    UserChords => 0x04,
    ChordMap => 0x0E,
    Custom1 => 0x05,
    Custom2 => 0x06,
    Custom3 => 0x07,
    Custom4 => 0x08,
    ArpPattern => 0x0D,
});
