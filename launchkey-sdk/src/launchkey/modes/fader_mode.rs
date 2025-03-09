#[derive(Debug, Clone)]
pub enum FaderMode {
    /// Controls track volume levels.
    Volume,
    /// User-defined fader mappings (slot 1).
    Custom1,
    /// User-defined fader mappings (slot 2).
    Custom2,
    /// User-defined fader mappings (slot 3).
    Custom3,
    /// User-defined fader mappings (slot 4).
    Custom4,
}

impl FaderMode {
    pub fn as_bytes(&self) -> Vec<u8> {
        let value = match self {
            FaderMode::Volume => 0x01,
            FaderMode::Custom1 => 0x06,
            FaderMode::Custom2 => 0x07,
            FaderMode::Custom3 => 0x08,
            FaderMode::Custom4 => 0x09,
        };
        vec![0xB6, 0x1F, value]
    }
}
