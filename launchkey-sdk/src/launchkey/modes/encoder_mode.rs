#[derive(Debug, Clone)]
pub enum EncoderMode {
    Plugin,
    Mixer,
    Sends,
    Transport,
    Custom1,
    Custom2,
    Custom3,
    Custom4,
}

impl EncoderMode {
    pub fn as_bytes(&self) -> Vec<u8> {
        let value = match self {
            EncoderMode::Plugin => 0x02,
            EncoderMode::Mixer => 0x01,
            EncoderMode::Sends => 0x04,
            EncoderMode::Transport => 0x05,
            EncoderMode::Custom1 => 0x06,
            EncoderMode::Custom2 => 0x07,
            EncoderMode::Custom3 => 0x08,
            EncoderMode::Custom4 => 0x09,
        };
        vec![0xB6, 0x1E, value]
    }
}