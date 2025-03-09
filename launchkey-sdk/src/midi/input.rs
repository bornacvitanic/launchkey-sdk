use crate::midi::to_hex::ToHexString;
use midir::{MidiInput, MidiInputConnection};
use std::io;
use std::io::Write;
use std::sync::mpsc;
use wmidi::MidiMessage;

pub fn get_named_midi_ports(midi_in: &MidiInput) -> Vec<(usize, String)> {
    midi_in
        .ports()
        .iter()
        .enumerate()
        .filter_map(|(i, port)| midi_in.port_name(port).ok().map(|name| (i, name)))
        .collect()
}

pub fn select_port(ports: &[(usize, String)]) -> Option<usize> {
    println!("Select a MIDI input port:");
    for (i, name) in ports.iter().enumerate() {
        println!("{}: {}", i, name.1);
    }

    print!("Enter port index: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<usize>().ok().and_then(
        |idx| {
            if idx < ports.len() {
                Some(idx)
            } else {
                None
            }
        },
    )
}

pub fn connect_to_port(
    midi_in: MidiInput,
    port_index: usize,
    tx: mpsc::Sender<Vec<u8>>,
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
                tx.send(message.to_vec()).unwrap();
            },
            (),
        )
        .map_err(|_| "Failed to connect to MIDI port".to_string())?;

    Ok(conn_in)
}

pub fn log_midi_message(message: MidiMessage) {
    match message {
        MidiMessage::NoteOff(_, note, _) => println!("Note Off: {}", note),
        MidiMessage::NoteOn(_, note, velocity) => {
            println!("Note On: {} (Velocity: {:?})", note, velocity)
        }
        MidiMessage::PolyphonicKeyPressure(_, note, velocity) => println!(
            "Polyphonic Key Pressure: {} (Velocity: {:?})",
            note, velocity
        ),
        MidiMessage::ControlChange(_, control_function, value) => println!(
            "Control Change: Controller {:?} Value {:?}",
            control_function, value
        ),
        MidiMessage::ProgramChange(_, program_number) => {
            println!("Program Change: Program {:?}", program_number)
        }
        MidiMessage::ChannelPressure(channel, velocity) => {
            println!("Channel Pressure: {:?} (Velocity: {:?})", channel, velocity)
        }
        MidiMessage::PitchBendChange(_, pitch_bend) => println!("Pitch Bend: {:?}", pitch_bend),
        MidiMessage::SysEx(data) => {
            println!("SysEx Message: {}", data.to_hex_string());
        }
        _ => {}
    }
}
