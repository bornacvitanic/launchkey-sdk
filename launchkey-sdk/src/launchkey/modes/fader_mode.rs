use crate::bidirectional_enum_mappings;

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

bidirectional_enum_mappings!(FaderMode, u8, {
    Volume => 0x01,
    Custom1 => 0x06,
    Custom2 => 0x07,
    Custom3 => 0x08,
    Custom4 => 0x09,
});