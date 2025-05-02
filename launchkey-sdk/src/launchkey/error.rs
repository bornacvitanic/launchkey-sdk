use midir::{ConnectError, MidiOutput};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LaunchkeyInitError {
    #[error("MIDI Initialization error: {0}")]
    Init(#[from] midir::InitError),

    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    #[error("MIDI connect error: {0}")]
    MidiConnectError(ConnectError<MidiOutput>),
}

#[derive(Debug, Error)]
#[error("MIDI send error: {source}")]
pub struct MidiSendError {
    #[from]
    pub source: midir::SendError,
}

#[derive(Debug, Error)]
pub enum LaunchkeyError {
    #[error("Launchkey Initialization error: {0}")]
    LaunchkeyInit(#[from] LaunchkeyInitError),

    #[error("MIDI send failed: {0}")]
    MidiSend(#[from] MidiSendError),
}