#[derive(Debug, Clone)]
pub enum FaderMode {
    Volume,
    Custom1,
    Custom2,
    Custom3,
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
