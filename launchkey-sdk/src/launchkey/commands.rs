const ENABLE_DAW_MODE: [u8; 3] = [0x9F, 0x0C, 0x7F];
const DISABLE_DAW_MODE: [u8; 3] = [0x9F, 0x0C, 0x00];
const SYSEX_TERMINATOR: u8 = 0xF7;

pub enum LaunchKeyCommand {
    EnableDAWMode,
    DisableDAWMode,
    SetPadMode(PadMode),
    SetEncoderMode(EncoderMode),
    SetFaderMode(FaderMode),
    SetPadColor {
        pad_id: u8,
        mode: LEDMode,
        color: Color,
    },
    SetPadCustomColor {
        pad_id: u8,
        r: u8,
        g: u8,
        b: u8,
    },
    ConfigureDisplay {
        target: DisplayTarget,
        config: DisplayConfig,
    },
    SetScreenText {
        target: DisplayTarget,
        field: u8,
        text: String,
    },
    SendScreenBitmap {
        target: DisplayTarget,
        bitmap_data: [u8; 1216],
    },
}

impl LaunchKeyCommand {
    pub(crate) fn as_bytes(&self, sku: &LaunchKeySku) -> Vec<u8> {
        let header = sku.sys_ex_header();
        match self {
            LaunchKeyCommand::EnableDAWMode => ENABLE_DAW_MODE.to_vec(),
            LaunchKeyCommand::DisableDAWMode => DISABLE_DAW_MODE.to_vec(),
            LaunchKeyCommand::SetPadMode(mode) => mode.as_bytes(),
            LaunchKeyCommand::SetEncoderMode(mode) => mode.as_bytes(),
            LaunchKeyCommand::SetFaderMode(mode) => mode.as_bytes(),
            // New commands for pad LEDs
            LaunchKeyCommand::SetPadColor {
                pad_id,
                mode,
                color,
            } => {
                let channel = match mode {
                    LEDMode::Stationary => 0x90,
                    LEDMode::Flashing => 0x91,
                    LEDMode::Pulsing => 0x92,
                };
                vec![channel, *pad_id, (*color) as u8]
            }
            LaunchKeyCommand::SetPadCustomColor { pad_id, r, g, b } => {
                let mut data = header.to_vec();
                data.extend_from_slice(&[0x01, 0x43, *pad_id, *r, *g, *b]);
                data.push(SYSEX_TERMINATOR);
                data
            }

            // New commands for screen control
            LaunchKeyCommand::ConfigureDisplay { target, config } => {
                let mut data = header.to_vec();
                data.extend_from_slice(&[0x04, (*target).into(), (*config).into()]);
                data.push(SYSEX_TERMINATOR);
                data
            }
            LaunchKeyCommand::SetScreenText {
                target,
                field,
                text,
            } => {
                let mut data = header.to_vec();
                data.extend_from_slice(&[0x06, (*target).into(), *field]);
                data.extend(text.as_bytes());
                data.push(SYSEX_TERMINATOR);
                data
            }
            LaunchKeyCommand::SendScreenBitmap {
                target,
                bitmap_data,
            } => {
                let mut data = header.to_vec();
                data.extend_from_slice(&[0x09, (*target).into()]);
                data.extend_from_slice(bitmap_data);
                data.push(SYSEX_TERMINATOR);
                data
            }
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug, Clone, Copy)]
pub enum LEDMode {
    Stationary,
    Flashing,
    Pulsing,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Off = 0x00,
    LowRed = 0x07,
    MediumRed = 0x06,
    HighRed = 0x05,
    LowYellow = 0x0F,
    MediumYellow = 0x0E,
    HighYellow = 0x0D,
    LowGreen = 0x17,
    MediumGreen = 0x16,
    HighGreen = 0x15,
}

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

#[derive(Debug)]
pub enum PadMode {
    Drum,
    DAW,
    UserChords,
    Custom(u8),
}

impl PadMode {
    pub fn as_bytes(&self) -> Vec<u8> {
        let value = match self {
            PadMode::Drum => 0x01,
            PadMode::DAW => 0x02,
            PadMode::UserChords => 0x04,
            PadMode::Custom(value) => *value,
        };
        vec![0xB6, 0x1D, value]
    }
}

#[derive(Debug)]
pub enum EncoderMode {
    Plugin,
    Mixer,
    Sends,
    Transport,
    Custom(u8),
}

impl EncoderMode {
    pub fn as_bytes(&self) -> Vec<u8> {
        let value = match self {
            EncoderMode::Plugin => 0x02,
            EncoderMode::Mixer => 0x01,
            EncoderMode::Sends => 0x04,
            EncoderMode::Transport => 0x05,
            EncoderMode::Custom(value) => *value,
        };
        vec![0xB6, 0x1E, value]
    }
}

#[derive(Debug)]
pub enum FaderMode {
    Volume,
    Custom(u8),
}

impl FaderMode {
    pub fn as_bytes(&self) -> Vec<u8> {
        let value = match self {
            FaderMode::Volume => 0x01,
            FaderMode::Custom(value) => *value,
        };
        vec![0xB6, 0x1F, value]
    }
}
