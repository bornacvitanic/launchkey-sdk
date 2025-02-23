use ctrlc;
use launchkey_sdk::launchkey::manager::LaunchkeyManager;
use launchkey_sdk::midi::events::MidiEvent;
use launchkey_sdk::midi::input::{connect_to_port, list_midi_ports};
use midir::{Ignore, MidiInput};
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;

fn main() {
    let mut midi_in = MidiInput::new("MIDI Listener").unwrap();
    midi_in.ignore(Ignore::None);

    // List available MIDI ports
    let ports = list_midi_ports(&midi_in);
    if ports.is_empty() {
        println!("No MIDI input ports found.");
        return;
    }

    // Let the user select a port
    let port_index = match select_port(&ports) {
        Some(idx) => idx,
        None => {
            println!("Invalid port selection.");
            return;
        }
    };

    println!("Connecting to port: {}", ports[port_index].1);

    // Set up MIDI output and enable DAW mode
    let mut launchkey_manager = match LaunchkeyManager::default() {
        Ok(cleanup) => cleanup,
        Err(err) => {
            println!("Error setting up Launchkey Manager: {}", err);
            return;
        }
    };

    // Set up DAW mode
    launchkey_manager.setup_daw_mode().expect("Failed to set up DAW mode.");

    // Set up a channel for communication between threads
    let (tx, rx) = mpsc::channel();

    // Connect to the selected port
    let _conn_in = match connect_to_port(midi_in, port_index, tx) {
        Ok(conn) => conn,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    println!("Listening for MIDI messages...");

    // Atomic boolean to signal termination
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    // Set up Ctrl+C handler
    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
        println!("Exiting...");
    })
    .expect("Error setting Ctrl+C handler");

    loop {
        if !running.load(Ordering::SeqCst) {
            break; // Exit the loop if the termination signal is received
        }

        match rx.try_recv() {
            Ok(event) => handle_midi_event(event), // Process the received MIDI event
            Err(mpsc::TryRecvError::Empty) => {}   // No message available, continue looping
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("MIDI channel disconnected.");
                break; // Exit the loop if the channel is closed
            }
        }

        // Optionally, add a small sleep to avoid busy-waiting
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    // Explicitly disable DAW mode before dropping the cleanup guard
    if let Err(err) = launchkey_manager.disable_daw_mode() {
        eprintln!("Failed to disable DAW mode during cleanup: {}", err);
    }
}

fn handle_midi_event(event: MidiEvent) {
    match event {
        MidiEvent::NoteOn { note, velocity } => {
            println!("Note On: {} (Velocity: {})", note, velocity);
        }
        MidiEvent::NoteOff { note } => {
            println!("Note Off: {}", note);
        }
    }
}

fn select_port(ports: &[(usize, String)]) -> Option<usize> {
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