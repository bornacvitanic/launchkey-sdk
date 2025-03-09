#[derive(Debug, Clone)]
pub enum EncoderMode {
    /// Controls plugin parameters.
    Plugin,
    /// Adjusts mixer levels, such as volume and pan.
    Mixer,
    /// Controls send levels for effects and routing.
    Sends,
    /// Handles transport functions like playback control.
    Transport,
    /// User-defined encoder mappings for custom controls (slot 1).
    Custom1,
    /// User-defined encoder mappings for custom controls (slot 2).
    Custom2,
    /// User-defined encoder mappings for custom controls (slot 3).
    Custom3,
    /// User-defined encoder mappings for custom controls (slot 4).
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
