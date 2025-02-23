const ENABLE_DAW_MODE: [u8; 3] = [0x9F, 0x0C, 0x7F];
const DISABLE_DAW_MODE: [u8; 3] = [0x9F, 0x0C, 0x00];

pub enum LaunchKeyCommand {
    EnableDAWMode,
    DisableDAWMode,
    SetPadMode(PadMode),
    SetEncoderMode(EncoderMode),
    SetFaderMode(FaderMode),
}

impl LaunchKeyCommand {
    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        match self {
            LaunchKeyCommand::EnableDAWMode => ENABLE_DAW_MODE.to_vec(),
            LaunchKeyCommand::DisableDAWMode => DISABLE_DAW_MODE.to_vec(),
            LaunchKeyCommand::SetPadMode(mode) => mode.as_bytes(),
            LaunchKeyCommand::SetEncoderMode(mode) => mode.as_bytes(),
            LaunchKeyCommand::SetFaderMode(mode) => mode.as_bytes(),
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