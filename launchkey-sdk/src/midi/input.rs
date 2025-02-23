use crate::midi::events::MidiEvent;
use midir::{MidiInput, MidiInputConnection};
use std::sync::mpsc;

pub fn list_midi_ports(midi_in: &MidiInput) -> Vec<(usize, String)> {
    midi_in
        .ports()
        .iter()
        .enumerate()
        .filter_map(|(i, port)| midi_in.port_name(port).ok().map(|name| (i, name)))
        .collect()
}

pub fn connect_to_port(
    midi_in: MidiInput,
    port_index: usize,
    tx: mpsc::Sender<MidiEvent>,
) -> Result<MidiInputConnection<()>, String> {
    let ports = midi_in.ports();
    let port = ports
        .get(port_index)
        .ok_or_else(|| "Invalid port index".to_string())?;

    // Connect to the MIDI port
    let conn_in = midi_in
        .connect(
            port,
            "midi-input",
            move |_timestamp, message, _context| {
                if let Some(event) = MidiEvent::parse(message) {
                    tx.send(event).unwrap();
                }
            },
            (),
        )
        .map_err(|_| "Failed to connect to MIDI port".to_string())?;

    Ok(conn_in)
}