use crate::launchkey::bitmap::LaunchkeyBitmap;
use crate::launchkey::commands::LaunchkeyCommand;
use crate::launchkey::constants::{LaunchKeySku, DISABLE_DAW_MODE, ENABLE_DAW_MODE};
use crate::launchkey::modes::encoder_mode::EncoderMode;
use crate::launchkey::modes::fader_mode::FaderMode;
use crate::launchkey::modes::pad_mode::PadMode;
use crate::launchkey::surface::display::{Arrangement, GlobalDisplayTarget};
use midir::{MidiOutput, MidiOutputPort};
use std::any::TypeId;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use log::{debug, error, info, log};
use crate::launchkey::error::{LaunchkeyError, LaunchkeyInitError, MidiSendError};
use crate::midi::to_hex::ToHexString;

/// Marker trait for state-dependent behavior in [`LaunchkeyManager`].
pub trait LaunchKeyState {}

/// Marker type representing the Launchkey being in DAW mode.
pub struct DAWMode;
impl LaunchKeyState for DAWMode {}

/// Marker type representing the Launchkey being in standalone mode.
pub struct StandaloneMode;
impl LaunchKeyState for StandaloneMode {}

/// A stateful manager for sending MIDI commands to a Launchkey device.
/// It is generic over a state (['DAWMode'] or ['StandaloneMode']) to enforce correct usage.
pub struct LaunchkeyManager<S: LaunchKeyState + 'static> {
    conn_out: Rc<RefCell<midir::MidiOutputConnection>>,
    sku: LaunchKeySku,
    state: PhantomData<S>,
}

impl<S: LaunchKeyState> LaunchkeyManager<S> {
    /// Sends a MIDI command to the Launchkey.
    fn _send_command(&mut self, command: LaunchkeyCommand) -> Result<(), MidiSendError> {
        match command {
            LaunchkeyCommand::SetPadMode(ref pad_mode) => {
                match pad_mode {
                    PadMode::Drum => self._send_command(LaunchkeyCommand::SetDrumDAWMode(false)),
                    PadMode::DrumDAW => self._send_command(LaunchkeyCommand::SetDrumDAWMode(true)),
                    _ => Ok(()),
                }?;
                self._send_bytes(&command.as_bytes(&self.sku))
            }
            command => self._send_bytes(&command.as_bytes(&self.sku)),
        }
    }

    /// Sends raw byte command to the Launchkey.
    fn _send_bytes(&mut self, bytes: &[u8]) -> Result<(), MidiSendError> {
        debug!("Sending MIDI message: {}", &bytes.to_hex_string());
        self.conn_out.borrow_mut().send(bytes)?;
        Ok(())
    }
}

impl LaunchkeyManager<StandaloneMode> {
    /// Creates a new ['LaunchkeyManager'] and connects to the specified output port.
    pub fn new(
        midi_out: MidiOutput,
        port: &MidiOutputPort,
        sku: LaunchKeySku,
    ) -> Result<Self, LaunchkeyInitError> {
        let conn_out = midi_out
            .connect(port, "launchkey-manager")
            .map_err(LaunchkeyInitError::MidiConnectError)?;
        Ok(Self {
            conn_out: Rc::new(RefCell::new(conn_out)),
            sku,
            state: PhantomData,
        })
    }

    /// Provides a default ['LaunchkeyManager'] instance.
    pub fn default() -> Result<Self, LaunchkeyInitError> {
        // Create MIDI output instance
        let midi_out = MidiOutput::new("Custom DAW")?;

        let ports = midi_out.ports();
        if ports.is_empty() {
            return Err(LaunchkeyInitError::DeviceNotFound("No MIDI output ports found".to_string()));
        }

        // Find the desired output port (e.g., "MIDIOUT2")
        info!("Looking for 'MIDIOUT2' among available MIDI ports...");
        let out_port = ports
            .iter()
            .find(|port| {
                midi_out
                    .port_name(port)
                    .unwrap_or_default()
                    .contains("MIDIOUT2")
            })
            .ok_or_else(|| LaunchkeyInitError::DeviceNotFound("Could not find MIDIOUT2 port".to_string()))?;

        Self::new(midi_out, out_port, LaunchKeySku::Mini)
    }

    /// Sends a command to set the global screen text in ['StandaloneMode'].
    pub fn set_screen_text_global(
        &mut self,
        target: GlobalDisplayTarget,
        arrangement: Arrangement,
    ) -> Result<(), MidiSendError> {
        self._send_command(LaunchkeyCommand::SetScreenTextGlobal {
            target,
            arrangement,
        })
    }

    /// Sends a command to display a screen bitmap in ['StandaloneMode'].
    pub fn send_screen_bitmap(
        &mut self,
        target: GlobalDisplayTarget,
        bitmap: LaunchkeyBitmap,
    ) -> Result<(), MidiSendError> {
        self._send_command(LaunchkeyCommand::SendScreenBitmap {
            target,
            bitmap: Box::new(bitmap),
        })
    }

    /// Enables DAW Mode and transitions the manager to ['DAWMode'].
    pub fn into_daw_mode(mut self) -> Result<LaunchkeyManager<DAWMode>, LaunchkeyError> {
        self._send_bytes(&ENABLE_DAW_MODE)?;
        info!("Transitioned to DAW mode");
        Ok(LaunchkeyManager {
            conn_out: self.conn_out.clone(),
            sku: self.sku.clone(),
            state: PhantomData,
        })
    }
}

impl LaunchkeyManager<DAWMode> {
    /// Disables DAW Mode and transitions the manager to ['StandaloneMode'].
    pub fn into_standalone_mode(
        mut self,
    ) -> Result<LaunchkeyManager<StandaloneMode>, LaunchkeyError> {
        self._send_bytes(&DISABLE_DAW_MODE)?;
        info!("Transitioned to Standalone mode");
        Ok(LaunchkeyManager {
            conn_out: self.conn_out.clone(),
            sku: self.sku.clone(),
            state: PhantomData,
        })
    }

    /// Sets the default mode for the Pads, Encoders and Faders on the Launchkey.
    pub fn setup_default_element_modes(&mut self) -> Result<(), LaunchkeyError> {
        self.send_command(LaunchkeyCommand::SetPadMode(PadMode::DAW))?;
        self.send_command(LaunchkeyCommand::SetEncoderMode(EncoderMode::Plugin))?;
        self.send_command(LaunchkeyCommand::SetFaderMode(FaderMode::Volume))?;

        Ok(())
    }

    /// Sends a MIDI command to the Launchkey.
    pub fn send_command(&mut self, command: LaunchkeyCommand) -> Result<(), MidiSendError> {
        debug!("Dispatching command: {:?}", command);
        self._send_command(command)
    }

    /// Sends multiple MIDI commands to the Launchkey.
    pub fn send_commands(&mut self, commands: &[LaunchkeyCommand]) -> Result<(), MidiSendError> {
        debug!("Sending {} MIDI commands to Launchkey", commands.len());
        for command in commands {
            self._send_command((*command).clone())?;
        }
        Ok(()) // Return Ok if all commands are successfully sent
    }
}

impl<S: LaunchKeyState + 'static> Drop for LaunchkeyManager<S> {
    fn drop(&mut self) {
        // Check if the current type is DawMode
        if TypeId::of::<S>() == TypeId::of::<DAWMode>() {
            if let Err(err) = self._send_bytes(&DISABLE_DAW_MODE) {
                error!("Failed to disable DAW mode during cleanup: {}", err);
            }
        }
    }
}