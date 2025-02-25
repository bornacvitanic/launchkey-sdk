#[derive(Debug)]
pub enum PadMode {
    Drum,
    DAW,
    UserChords,
    Custom1,
    Custom2,
    Custom3,
    Custom4,
    ArpPattern,
    ChordMap,
}

impl PadMode {
    pub fn as_bytes(&self) -> Vec<u8> {
        let value = match self {
            PadMode::Drum => 0x01,
            PadMode::DAW => 0x02,
            PadMode::UserChords => 0x04,
            PadMode::Custom1 => 0x05,
            PadMode::Custom2 => 0x06,
            PadMode::Custom3 => 0x07,
            PadMode::Custom4 => 0x08,
            PadMode::ArpPattern => 0x0D,
            PadMode::ChordMap => 0x0E,
        };
        vec![0xB6, 0x1D, value]
    }
}
