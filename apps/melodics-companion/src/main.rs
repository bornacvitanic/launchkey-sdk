use launchkey_sdk::launchkey::manager::LaunchkeyManager;
use launchkey_sdk::launchkey::surface::buttons::{ButtonState, LaunchKeyButton};
use launchkey_sdk::midi::input::{connect_to_port, get_named_midi_ports};
use midir::{Ignore, MidiInput, MidiInputConnection};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use enigo::{Direction, Enigo, Keyboard};
use wmidi::{Channel, MidiMessage};

fn main() {
    // Set up LaunchkeyManager with default configuration
    let launchkey_manager = match LaunchkeyManager::default() {
        Ok(manager) => manager,
        Err(err) => {
            println!("Error setting up LaunchkeyManager: {}", err);
            return;
        }
    };

    // Set up DAW mode
    let _launchkey_manager = match launchkey_manager.into_daw_mode() {
        Ok(daw_manager) => daw_manager,
        Err(err) => {
            println!("Error setting up DAW mode: {}", err);
            return;
        }
    };

    // Set up a channel for communication between threads
    let (tx, rx) = mpsc::channel();

    let midi_in = MidiInput::new("MIDI Listener").unwrap();
    let ports = get_named_midi_ports(&midi_in);

    let mut connections: Vec<MidiInputConnection<()>> = vec![];
    for (i, name) in ports.iter() {
        let mut midi_in = MidiInput::new(&format!("{} Listener", name)).unwrap();
        midi_in.ignore(Ignore::None);
        // Connect to the selected port
        let _conn_in = match connect_to_port(midi_in, *i, tx.clone()) {
            Ok(conn) => conn,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };
        connections.push(_conn_in);
    }

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

    let mut enigo = Enigo::new(&Default::default()).unwrap();

    loop {
        if !running.load(Ordering::SeqCst) {
            break; // Exit the loop if the termination signal is received
        }

        match rx.try_recv() {
            Ok(data) => {
                if let Ok(event) = MidiMessage::try_from(data.as_slice()) {
                    handle_button_presses(event, &mut enigo)
                }
            } // Process the received MIDI event
            Err(mpsc::TryRecvError::Empty) => {} // No message available, continue looping
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("MIDI channel disconnected.");
                break; // Exit the loop if the channel is closed
            }
        }

        // Optionally, add a small sleep to avoid busy-waiting
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn handle_button_presses(message: MidiMessage, enigo: &mut Enigo) {
    match message {
        MidiMessage::ControlChange(Channel::Ch1, control_function, value) => {
            match (LaunchKeyButton::from_value(control_function.0.into()), ButtonState::try_from(value)) {
                (Some(LaunchKeyButton::Play), Ok(ButtonState::Pressed)) => enigo.key(enigo::Key::Space, Direction::Click).unwrap(),
                (Some(LaunchKeyButton::Record), Ok(ButtonState::Pressed)) => enigo.key(enigo::Key::Escape, Direction::Click).unwrap(),
                _ => {}
            }
        }
        _ => {}
    }
}