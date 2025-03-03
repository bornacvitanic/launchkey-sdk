use ctrlc;
use launchkey_sdk::launchkey::colors::CommonColor;
use launchkey_sdk::launchkey::commands::LaunchKeyCommand;
use launchkey_sdk::launchkey::manager::LaunchkeyManager;
use launchkey_sdk::launchkey::surface::buttons::{Brightness, LaunchKeyButton, MiniButton};
use launchkey_sdk::launchkey::surface::display::{Arrangement, DisplayTarget, ModeNameTarget, TemporaryTarget};
use launchkey_sdk::launchkey::surface::encoders::Encoder;
use launchkey_sdk::launchkey::surface::pads::{LEDMode, Pad, PadInMode};
use launchkey_sdk::midi::input::{connect_to_port, list_midi_ports};
use midir::{Ignore, MidiInput, MidiInputConnection};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use wmidi::MidiMessage;
use launchkey_sdk::midi::input;

fn main() {
    // Set up LaunchkeyManager with default configuration
    let mut launchkey_manager = match LaunchkeyManager::default() {
        Ok(manager) => manager,
        Err(err) => {
            println!("Error setting up LaunchkeyManager: {}", err);
            return;
        }
    };

    // Set up DAW mode
    if let Err(err) = launchkey_manager.setup_daw_mode() {
        println!("Error setting up DAW mode: {}", err);
        return;
    }

    // Set Play Button brightness
    launchkey_manager
        .send_command(LaunchKeyCommand::SetButtonBrightness {
            launch_key_button: LaunchKeyButton::Mini(MiniButton::Play),
            brightness: Brightness::max(),
        })
        .unwrap();

    // Set a pads LED to green
    for pad in Pad::all() {
        launchkey_manager
            .send_command(LaunchKeyCommand::SetPadColor {
                pad_in_mode: PadInMode::DAW(pad),
                mode: LEDMode::Stationary,
                color_palette_index: CommonColor::BrightGreen.into(),
            })
            .unwrap();
    }

    // Set a pad's LED to custom RGB color
    launchkey_manager
        .send_command(LaunchKeyCommand::SetPadCustomColor {
            pad_in_mode: PadInMode::DAW(Pad::PlugIn),
            color: CommonColor::BrightCyan.into(),
        })
        .unwrap();

    // Enable DAW Drum Mode
    launchkey_manager.enable_daw_drum_mode().unwrap();

    // Set a pad to HighGreen in DAW Drum Mode
    launchkey_manager
        .send_command(LaunchKeyCommand::SetPadColor {
            pad_in_mode: PadInMode::Drum(Pad::Mixer),
            mode: LEDMode::Stationary,
            color_palette_index: CommonColor::BrightRed.into(),
        })
        .unwrap();

    // Configure and set text on the screen
    launchkey_manager.send_command(
        LaunchKeyCommand::SetScreenTextArrangement {
            target: DisplayTarget::Stationary,
            arrangement: Arrangement::NameValue(
                "Custom DAW".to_string(),
                "Hello, World!".to_string()
            )
        }
    ).unwrap();

    // Customize names for all encoders
    for (index, encoder) in Encoder::all().enumerate() {
        launchkey_manager
            .send_command(LaunchKeyCommand::SetScreenText {
                target: DisplayTarget::Temporary(TemporaryTarget::Encoder(encoder)),
                field: 0,
                text: format!("Custom Encoder {}", index + 1),
            })
            .unwrap();
    }

    // Customizing PAD mode names
    for mode_name_target in ModeNameTarget::all() {
        launchkey_manager
            .send_command(LaunchKeyCommand::SetScreenText {
                target: DisplayTarget::ModeName(mode_name_target),
                field: 0,
                text: format!("Custom {}", mode_name_target),
            })
            .unwrap();
    }

    // Send a bitmap to the screen
    let mut bitmap_data = [0u8; 1216];
    bitmap_data = bitmap_data.map(|_e| 0x12);
    // Populate the bitmap_data array with your custom bitmap
    launchkey_manager
        .send_command(LaunchKeyCommand::SendScreenBitmap {
            target: DisplayTarget::GlobalTemporary,
            bitmap_data,
        })
        .unwrap();

    // Set up a channel for communication between threads
    let (tx, rx) = mpsc::channel();

    let midi_in = MidiInput::new("MIDI Listener").unwrap();
    let ports = list_midi_ports(&midi_in);
    println!("Select a MIDI input port:");
    let mut connections: Vec<MidiInputConnection<()>> = vec![];
    for (i, name) in ports.iter() {
        println!("{}: {}", i, name);
        let mut midi_in = MidiInput::new("MIDI Listener").unwrap();
        midi_in.ignore(Ignore::None);
        // Connect to the selected port
        let _conn_in = match connect_to_port(midi_in, *i, tx.clone()) {
            Ok(conn) => conn,
            Err(err) => {
                println!("Error: {}", err);
                return;
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

    loop {
        if !running.load(Ordering::SeqCst) {
            break; // Exit the loop if the termination signal is received
        }

        match rx.try_recv() {
            Ok(data) => {
                if let Ok(event) = MidiMessage::try_from(data.as_slice()) {
                    input::log_midi_message(event);
                }
            } // Process the received MIDI event
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