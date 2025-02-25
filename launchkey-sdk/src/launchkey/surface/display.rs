#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum DisplayTarget {
    Temporary(u8), // 00h - 1Fh
    Stationary,
    GlobalTemporary,
    DAWPadModeName,
    DAWDrumPadModeName,
    MixerEncoderModeName,
    PluginEncoderModeName,
    SendsEncoderModeName,
    TransportEncoderModeName,
    VolumeFaderModeName,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum DisplayConfig {
    Cancel,
    Trigger,
    Arrangement(u8),
}

impl From<DisplayTarget> for u8 {
    fn from(target: DisplayTarget) -> Self {
        match target {
            DisplayTarget::Temporary(idx) => idx,
            DisplayTarget::Stationary => 0x20,
            DisplayTarget::GlobalTemporary => 0x21,
            DisplayTarget::DAWPadModeName => 0x22,
            DisplayTarget::DAWDrumPadModeName => 0x23,
            DisplayTarget::MixerEncoderModeName => 0x24,
            DisplayTarget::PluginEncoderModeName => 0x25,
            DisplayTarget::SendsEncoderModeName => 0x26,
            DisplayTarget::TransportEncoderModeName => 0x27,
            DisplayTarget::VolumeFaderModeName => 0x28,
        }
    }
}

impl From<DisplayConfig> for u8 {
    fn from(config: DisplayConfig) -> Self {
        match config {
            DisplayConfig::Cancel => 0x00,
            DisplayConfig::Trigger => 0x7F,
            DisplayConfig::Arrangement(arrangement) => arrangement,
        }
    }
}
