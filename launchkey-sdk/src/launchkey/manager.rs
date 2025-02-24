use crate::launchkey::commands::{EncoderMode, FaderMode, LaunchKeyCommand, LaunchKeySku, PadMode};
use midir::{MidiOutput, MidiOutputPort};
use std::fmt::Write;

pub struct LaunchkeyManager {
    conn_out: midir::MidiOutputConnection,
    sku: LaunchKeySku,
    in_daw_drum_mode: bool,
}

impl LaunchkeyManager {
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
            conn_out,
            sku,
            in_daw_drum_mode: false,
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

    /// Sends a MIDI command to the Launchkey.
    pub fn send_command(&mut self, command: LaunchKeyCommand) -> Result<(), midir::SendError> {
        let bytes = command.as_bytes(&self.sku);
        // Print the bytes in hexadecimal format
        let mut hex_string = String::new();
        for byte in &bytes {
            let _ = &hex_string.write_str(&format!("{:02X} ", byte));
        }
        println!("Sending MIDI message: {}", hex_string.trim_end());
        self.conn_out.send(&bytes)?;
        Ok(())
    }

    /// Sets up DAW mode on the Launchkey.
    pub fn setup_daw_mode(&mut self) -> Result<(), midir::SendError> {
        self.send_command(LaunchKeyCommand::EnableDAWMode)?;
        self.send_command(LaunchKeyCommand::SetPadMode(PadMode::DAW))?;
        self.send_command(LaunchKeyCommand::SetEncoderMode(EncoderMode::Plugin))?;
        self.send_command(LaunchKeyCommand::SetFaderMode(FaderMode::Volume))?;

        Ok(())
    }

    /// Disables DAW mode on the Launchkey.
    pub fn disable_daw_mode(&mut self) -> Result<(), midir::SendError> {
        self.send_command(LaunchKeyCommand::DisableDAWMode)
    }

    /// Switch the Launchkey to DAW Drum Mode
    pub fn enable_daw_drum_mode(&mut self) -> Result<(), midir::SendError> {
        let message = [0xB6, 0x54, 0x01]; // Channel 7, CC 84, Value 1
        self.conn_out.send(&message)?;
        self.in_daw_drum_mode = true;
        println!("Enabled DAW Drum Mode");
        Ok(())
    }

    /// Switch the Launchkey back to Standalone Drum Mode
    pub fn disable_daw_drum_mode(&mut self) -> Result<(), midir::SendError> {
        let message = [0xB6, 0x54, 0x00]; // Channel 7, CC 84, Value 0
        self.conn_out.send(&message)?;
        self.in_daw_drum_mode = false;
        println!("Disabled DAW Drum Mode (returned to Standalone)");
        Ok(())
    }
}

impl Drop for LaunchkeyManager {
    fn drop(&mut self) {
        if let Err(err) = self.disable_daw_mode() {
            eprintln!("Failed to disable DAW mode during cleanup: {}", err);
        }
    }
}
