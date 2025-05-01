use launchkey_sdk::launchkey::bitmap::LaunchkeyBitmap;
use launchkey_sdk::launchkey::colors::CommonColor;
use launchkey_sdk::launchkey::commands::LaunchkeyCommand;
use launchkey_sdk::launchkey::constants::{ENCODER_MODE_CC, PAD_MODE_CC};
use launchkey_sdk::launchkey::manager::LaunchkeyManager;
use launchkey_sdk::launchkey::modes::encoder_mode::EncoderMode;
use launchkey_sdk::launchkey::modes::pad_mode::PadMode;
use launchkey_sdk::launchkey::surface::buttons::{Brightness, ButtonState, LaunchKeyButton};
use launchkey_sdk::launchkey::surface::display::{
    Arrangement, ContextualDisplayTarget, GlobalDisplayTarget, ModeNameTarget, TemporaryTarget,
};
use launchkey_sdk::launchkey::surface::encoders::Encoder;
use launchkey_sdk::launchkey::surface::pads::{LEDMode, Pad, PadInMode};
use launchkey_sdk::midi::input;
use launchkey_sdk::midi::input::{connect_all_midi_ports, ControlFunctionExt, get_named_midi_ports};
use midir::{MidiInput};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use wmidi::{Channel, MidiMessage};

fn main() {
    // Set up LaunchkeyManager with default configuration
    let launchkey_manager = LaunchkeyManager::default().unwrap_or_else(|err| {
        println!("Error setting up LaunchkeyManager: {}", err);
        std::process::exit(1);
    });

    // Set up DAW mode
    let mut launchkey_manager = launchkey_manager.into_daw_mode().unwrap_or_else(|err| {
        println!("Error setting up DAW mode: {}", err);
        std::process::exit(1);
    });

    // Set Play Button brightness
    launchkey_manager
        .send_command(LaunchkeyCommand::SetButtonBrightness {
            launch_key_button: LaunchKeyButton::Play,
            brightness: Brightness::max(),
        })
        .unwrap();

    // Set a pads LED to green
    for pad in Pad::all() {
        launchkey_manager
            .send_command(LaunchkeyCommand::SetPadColor {
                pad_in_mode: PadInMode::DAW(pad),
                mode: LEDMode::Stationary,
                color_palette_index: CommonColor::BrightGreen.into(),
            })
            .unwrap();
    }

    // Set a pad's LED to custom RGB color
    launchkey_manager
        .send_command(LaunchkeyCommand::SetPadCustomColor {
            pad_in_mode: PadInMode::DAW(Pad::PlugIn),
            color: CommonColor::BrightCyan.into(),
        })
        .unwrap();

    // Enable DAW Drum Mode
    launchkey_manager
        .send_command(LaunchkeyCommand::SetDrumDAWMode(true))
        .unwrap();

    // Set a pad to HighGreen in DAW Drum Mode
    launchkey_manager
        .send_command(LaunchkeyCommand::SetPadColor {
            pad_in_mode: PadInMode::Drum(Pad::Mixer),
            mode: LEDMode::Stationary,
            color_palette_index: CommonColor::BrightRed.into(),
        })
        .unwrap();

    // Configure and set text on the screen
    launchkey_manager
        .send_command(LaunchkeyCommand::SetScreenTextGlobal {
            target: GlobalDisplayTarget::Stationary,
            arrangement: Arrangement::NameValue(
                "Custom DAW".to_string(),
                "Hello, World!".to_string(),
            ),
        })
        .unwrap();

    launchkey_manager
        .send_command(LaunchkeyCommand::SetScreenTextGlobal {
            target: GlobalDisplayTarget::Temporary,
            arrangement: Arrangement::NameValue("Custom DAW".to_string(), "Temporary".to_string()),
        })
        .unwrap();

    // Customize names for all encoders
    for (index, encoder) in Encoder::all().enumerate() {
        launchkey_manager
            .send_command(LaunchkeyCommand::SetScreenTextContextual {
                target: ContextualDisplayTarget::Temporary(TemporaryTarget::Encoder(encoder)),
                text: format!("Custom Encoder {}", index + 1),
            })
            .unwrap();
    }

    // Customizing PAD mode names
    for mode_name_target in ModeNameTarget::all() {
        launchkey_manager
            .send_command(LaunchkeyCommand::SetScreenTextContextual {
                target: ContextualDisplayTarget::ModeName(mode_name_target),
                text: format!("Custom {}", mode_name_target),
            })
            .unwrap();
    }

    // Send a bitmap to the screen
    let img = image::open("logo.png").map_err(|e| e.to_string()).unwrap();
    let bitmap = LaunchkeyBitmap::from_image(img, 128).unwrap();
    launchkey_manager
        .send_command(LaunchkeyCommand::SendScreenBitmap {
            target: GlobalDisplayTarget::Temporary,
            bitmap: Box::new(bitmap),
        })
        .unwrap();

    // Set up a channel for communication between threads
    let (tx, rx) = mpsc::channel();

    let midi_in = MidiInput::new("MIDI Listener").unwrap();
    let ports = get_named_midi_ports(&midi_in);
    let _connections = connect_all_midi_ports(&ports, tx.clone());

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
                    input::log_midi_message(event.clone());
                    interpret_launchkey_midi(event)
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

fn interpret_launchkey_midi(message: MidiMessage) {
    match message {
        MidiMessage::ControlChange(Channel::Ch1, control_function, value) => {
            if let Some(button) = LaunchKeyButton::from_value(control_function.0.into()) {
                let state = match ButtonState::try_from(value) {
                    Ok(state) => state,
                    Err(_) => {
                        println!(
                            "Launchkey Button: {:?} Unknown ControlValue: {:?}",
                            button, value
                        );
                        return;
                    }
                };
                println!("Launchkey Button: {:?} State: {:?}", button, state);
            }
        }

        MidiMessage::ControlChange(_, control_function, value)
            if control_function.equals_u8(ENCODER_MODE_CC) =>
        {
            println!(
                "Encoder Mode Change: {:?}",
                EncoderMode::from_value(value.into())
            );
        }

        MidiMessage::ControlChange(Channel::Ch16, control_function, value)
            if Encoder::from_value(control_function.0.into()).is_some() =>
        {
            if let Some((encoder, mode)) = Encoder::from_value(control_function.0.into()) {
                println!("{:?} ({:?}) Value Change: {:?}", encoder, mode, value);
            }
        }

        MidiMessage::ControlChange(_, control_function, value)
            if control_function.equals_u8(PAD_MODE_CC) =>
        {
            println!("Pad Mode Change: {:?}", PadMode::from_value(value.into()));
        }

        MidiMessage::NoteOn(_, note, _) => {
            if let Some((pad, mode)) = Pad::from_value(note as u8) {
                println!("Pad: {:?}, Mode: {:?}", pad, mode);
            }
        }

        _ => {}
    }
}