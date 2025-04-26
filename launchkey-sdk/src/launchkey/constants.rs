// DAW Mode Constants
pub const ENABLE_DAW_MODE: [u8; 3] = [0x9F, 0x0C, 0x7F];
pub const DISABLE_DAW_MODE: [u8; 3] = [0x9F, 0x0C, 0x00];
pub const ENABLE_DRUM_DAW_MODE: [u8; 3] = [CC_CH7, 0x54, 0x01]; // Channel 7, CC 84, Value 1
pub const DISABLE_DRUM_DAW_MODE: [u8; 3] = [CC_CH7, 0x54, 0x00]; // Channel 7, CC 84, Value 0
                                                                 // MIDI Control Change Constants
pub const CC_CH7: u8 = 0xB6;
pub const PAD_MODE_CC: u8 = 0x1D;
pub const ENCODER_MODE_CC: u8 = 0x1E;
pub const FADER_MODE_CC: u8 = 0x1F;
// Button Brightness Constants
pub const BUTTON_BRIGHTNESS_OVERRIDE_CHANNEL: u8 = 0xB3;
// SysEx Constants
pub const SYSEX_TERMINATOR: u8 = 0xF7;
pub const CUSTOM_COLOR_COMMAND: (u8, u8) = (0x01, 0x43);
pub const CONFIGURE_DISPLAY_COMMAND: u8 = 0x04;
pub const SET_SCREEN_TEXT_COMMAND: u8 = 0x06;
pub const BITMAP_HEADER_BYTE: u8 = 0x09;

#[derive(Debug, Clone)]
pub enum LaunchKeySku {
    Regular,
    Mini,
}

impl LaunchKeySku {
    pub fn sys_ex_header(&self) -> [u8; 6] {
        let model_identifier = match self {
            LaunchKeySku::Regular => 0x14,
            LaunchKeySku::Mini => 0x13,
        };

        [0xF0, 0x00, 0x20, 0x29, 0x02, model_identifier]
    }
}