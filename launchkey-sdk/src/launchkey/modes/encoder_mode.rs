use crate::bidirectional_enum_mappings;

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

bidirectional_enum_mappings!(EncoderMode, u8, {
    Plugin => 0x02,
    Mixer => 0x01,
    Sends => 0x04,
    Transport => 0x05,
    Custom1 => 0x06,
    Custom2 => 0x07,
    Custom3 => 0x08,
    Custom4 => 0x09,
});
