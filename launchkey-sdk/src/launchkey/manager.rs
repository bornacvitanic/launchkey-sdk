use midir::{MidiOutput, MidiOutputPort};
use crate::launchkey::commands::{EncoderMode, FaderMode, LaunchKeyCommand, PadMode};

pub struct LaunchkeyManager {
    conn_out: midir::MidiOutputConnection,
}

impl LaunchkeyManager {
    /// Creates a new LaunchkeyManager and connects to the specified output port.
    pub fn new(midi_out: MidiOutput, port: &MidiOutputPort) -> Result<Self, String> {
        let conn_out = midi_out
            .connect(port, "launchkey-manager")
            .map_err(|_| "Failed to connect to MIDI output".to_string())?;
        Ok(Self { conn_out })
    }

    pub fn default() -> Result<LaunchkeyManager, String> {
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

        LaunchkeyManager::new(midi_out, out_port)
    }

    /// Sends a MIDI command to the Launchkey.
    pub fn send_command(&mut self, command: LaunchKeyCommand) -> Result<(), midir::SendError> {
        let bytes = command.as_bytes();
        self.conn_out.send(&bytes)?;
        Ok(())
    }

    pub fn setup_daw_mode(&mut self) -> Result<(), midir::SendError> {
        self.send_command(LaunchKeyCommand::EnableDAWMode)?;
        self.send_command(LaunchKeyCommand::SetPadMode(PadMode::DAW))?;
        self.send_command(LaunchKeyCommand::SetEncoderMode(EncoderMode::Plugin))?;
        self.send_command(LaunchKeyCommand::SetFaderMode(FaderMode::Volume))?;

        Ok(())
    }

    pub fn disable_daw_mode(&mut self) -> Result<(), midir::SendError> {
        println!("Exiting and disabling DAW mode...");
        self.send_command(LaunchKeyCommand::DisableDAWMode)
    }
}

impl Drop for LaunchkeyManager {
    fn drop(&mut self) {
        if let Err(err) = self.disable_daw_mode() {
            eprintln!("Failed to disable DAW mode during cleanup: {}", err);
        }
    }
}