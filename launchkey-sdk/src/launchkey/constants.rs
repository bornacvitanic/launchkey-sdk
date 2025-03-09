pub const ENABLE_DAW_MODE: [u8; 3] = [0x9F, 0x0C, 0x7F];
pub const DISABLE_DAW_MODE: [u8; 3] = [0x9F, 0x0C, 0x00];
pub const CC_CH7: u8 = 0xB6; // Control Change on MIDI Channel 7
pub const ENABLE_DRUM_DAW_MODE: [u8; 3] = [CC_CH7, 0x54, 0x01]; // Channel 7, CC 84, Value 1
pub const DISABLE_DRUM_DAW_MODE: [u8; 3] = [CC_CH7, 0x54, 0x00]; // Channel 7, CC 84, Value 0
pub const PAD_MODE_CC: u8 = 0x1D;    // Control Change for Pad Mode
pub const ENCODER_MODE_CC: u8 = 0x1E; // Control Change for Encoder Mode
pub const FADER_MODE_CC: u8 = 0x1F;   // Control Change for Fader Mode
pub const BUTTON_BRIGHTNESS_OVERRIDE_CHANNEL: u8 = 0xB3;
pub const SYSEX_TERMINATOR: u8 = 0xF7;

#[derive(Debug, Clone)]
pub enum LaunchKeySku {
    Regular,
    Mini,
}

impl LaunchKeySku {
    pub fn sys_ex_header(&self) -> [u8; 6] {
        match self {
            LaunchKeySku::Regular => [0xF0, 0x00, 0x20, 0x29, 0x02, 0x14],
            LaunchKeySku::Mini => [0xF0, 0x00, 0x20, 0x29, 0x02, 0x13],
        }
    }
}