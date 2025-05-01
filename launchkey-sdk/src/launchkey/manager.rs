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

pub trait LaunchKeyState {}

pub struct DAWMode;
impl LaunchKeyState for DAWMode {}

pub struct StandaloneMode;
impl LaunchKeyState for StandaloneMode {}

pub struct LaunchkeyManager<S: LaunchKeyState + 'static> {
    conn_out: Rc<RefCell<midir::MidiOutputConnection>>,
    sku: LaunchKeySku,
    state: PhantomData<S>,
}

impl<S: LaunchKeyState> LaunchkeyManager<S> {
    /// Converts a slice of bytes into a formatted hexadecimal string.
    fn bytes_to_hex_string(&self, bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|byte| format!("{:02X} ", byte))
            .collect::<String>()
            .trim_end() // Remove trailing space
            .to_string()
    }

    /// Enables DAW mode on the Launchkey device by sending a MIDI message.
    fn _enable_daw_mode(&mut self) -> Result<(), midir::SendError> {
        println!("Enabling DAW Mode");
        self._send_bytes(ENABLE_DAW_MODE.to_vec())
    }

    /// Disables DAW mode on the Launchkey device by sending a MIDI message.
    fn _disable_daw_mode(&mut self) -> Result<(), midir::SendError> {
        println!("Disabling DAW Mode");
        self._send_bytes(DISABLE_DAW_MODE.to_vec())
    }

    /// Sends a MIDI command to the Launchkey.
    fn _send_command(&mut self, command: LaunchkeyCommand) -> Result<(), midir::SendError> {
        match command {
            LaunchkeyCommand::SetPadMode(ref pad_mode) => {
                match pad_mode {
                    PadMode::Drum => self._send_command(LaunchkeyCommand::SetDrumDAWMode(false)),
                    PadMode::DrumDAW => self._send_command(LaunchkeyCommand::SetDrumDAWMode(true)),
                    _ => Ok(()),
                }?;
                self._send_bytes(command.as_bytes(&self.sku))
            }
            command => self._send_bytes(command.as_bytes(&self.sku)),
        }
    }

    /// Sends raw byte command to the Launchkey.
    fn _send_bytes(&mut self, bytes: Vec<u8>) -> Result<(), midir::SendError> {
        let hex_string = self.bytes_to_hex_string(&bytes);
        println!("Sending MIDI message: {}", hex_string);
        self.conn_out.borrow_mut().send(&bytes)?;
        Ok(())
    }
}

impl LaunchkeyManager<StandaloneMode> {
    /// Creates a new LaunchkeyManager and connects to the specified output port.
    pub fn new(
        midi_out: MidiOutput,
        port: &MidiOutputPort,
        sku: LaunchKeySku,
    ) -> Result<Self, String> {
        let conn_out = midi_out
            .connect(port, "launchkey-manager")
            .map_err(|_| "Failed to connect to MIDI output".to_string())?;
        Ok(Self {
            conn_out: Rc::new(RefCell::new(conn_out)),
            sku,
            state: PhantomData,
        })
    }

    /// Provides a default LaunchkeyManager instance.
    pub fn default() -> Result<Self, String> {
        // Create MIDI output instance
        let midi_out = MidiOutput::new("Custom DAW")
            .map_err(|_| "Failed to create MIDI output".to_string())?;

        let ports = midi_out.ports();
        if ports.is_empty() {
            return Err("No MIDI output ports found".to_string());
        }

        // Find the desired output port (e.g., "MIDIOUT2")
        let out_port = ports
            .iter()
            .find(|port| {
                midi_out
                    .port_name(port)
                    .unwrap_or_default()
                    .contains("MIDIOUT2")
            })
            .ok_or_else(|| "Could not find MIDIOUT2 port".to_string())?;

        Self::new(midi_out, out_port, LaunchKeySku::Mini)
    }

    /// Sends a command to set the global screen text in Standalone Mode.
    pub fn set_screen_text_global(
        &mut self,
        target: GlobalDisplayTarget,
        arrangement: Arrangement,
    ) -> Result<(), midir::SendError> {
        self._send_command(LaunchkeyCommand::SetScreenTextGlobal {
            target,
            arrangement,
        })
    }

    /// Sends a command to display a screen bitmap in Standalone Mode.
    pub fn send_screen_bitmap(
        &mut self,
        target: GlobalDisplayTarget,
        bitmap: LaunchkeyBitmap,
    ) -> Result<(), midir::SendError> {
        self._send_command(LaunchkeyCommand::SendScreenBitmap {
            target,
            bitmap: Box::new(bitmap),
        })
    }

    /// Enables DAW Mode and transitions the manager to DAW mode.
    pub fn into_daw_mode(mut self) -> Result<LaunchkeyManager<DAWMode>, midir::SendError> {
        self._enable_daw_mode()?;
        Ok(LaunchkeyManager {
            conn_out: self.conn_out.clone(),
            sku: self.sku.clone(),
            state: PhantomData,
        })
    }
}

impl LaunchkeyManager<DAWMode> {
    /// Disables DAW Mode and transitions the manager to Standalone mode.
    pub fn into_standalone_mode(
        mut self,
    ) -> Result<LaunchkeyManager<StandaloneMode>, midir::SendError> {
        self._disable_daw_mode()?;
        Ok(LaunchkeyManager {
            conn_out: self.conn_out.clone(),
            sku: self.sku.clone(),
            state: PhantomData,
        })
    }

    /// Sets the default mode for the Pads, Encoders and Faders on the Launchkey.
    pub fn setup_default_element_modes(&mut self) -> Result<(), midir::SendError> {
        self.send_command(LaunchkeyCommand::SetPadMode(PadMode::DAW))?;
        self.send_command(LaunchkeyCommand::SetEncoderMode(EncoderMode::Plugin))?;
        self.send_command(LaunchkeyCommand::SetFaderMode(FaderMode::Volume))?;

        Ok(())
    }

    /// Sends a MIDI command to the Launchkey.
    pub fn send_command(&mut self, command: LaunchkeyCommand) -> Result<(), midir::SendError> {
        self._send_command(command)
    }

    /// Sends multiple MIDI commands to the Launchkey.
    pub fn send_commands(&mut self, commands: &[LaunchkeyCommand]) -> Result<(), midir::SendError> {
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
            if let Err(err) = self._disable_daw_mode() {
                eprintln!("Failed to disable DAW mode during cleanup: {}", err);
            }
        }
    }
}
