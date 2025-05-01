use enigo::{Direction, Enigo, Keyboard};
use launchkey_sdk::launchkey::colors::CommonColor;
use launchkey_sdk::launchkey::commands::LaunchkeyCommand;
use launchkey_sdk::launchkey::manager::{DAWMode, LaunchkeyManager};
use launchkey_sdk::launchkey::modes::pad_mode::PadMode;
use launchkey_sdk::launchkey::surface::buttons::{ButtonState, LaunchKeyButton};
use launchkey_sdk::launchkey::surface::display::{
    Arrangement, ContextualDisplayTarget, GlobalDisplayTarget, ModeNameTarget,
};
use launchkey_sdk::launchkey::surface::pads::{LEDMode, Pad, PadCCIndex, PadInMode};
use launchkey_sdk::midi::input;
use launchkey_sdk::midi::input::{connect_all_midi_ports, get_named_midi_ports};
use midir::MidiInput;
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

    // Set custom DAW mode name
    launchkey_manager
        .send_command(LaunchkeyCommand::SetScreenTextContextual {
            target: ContextualDisplayTarget::ModeName(ModeNameTarget::DAW),
            text: "Pad Color Setup".to_string(),
        })
        .unwrap();

    // Default to Pad Drum mode
    launchkey_manager
        .send_command(LaunchkeyCommand::SetPadMode(PadMode::DrumDAW))
        .unwrap();

    for pad in Pad::all() {
        launchkey_manager
            .send_command(LaunchkeyCommand::SetPadColor {
                pad_in_mode: PadInMode::Drum(pad),
                mode: LEDMode::Stationary,
                color_palette_index: CommonColor::Off.into(),
            })
            .unwrap();
    }

    // Set Stationary Text
    launchkey_manager
        .send_command(LaunchkeyCommand::SetScreenTextGlobal {
            target: GlobalDisplayTarget::Stationary,
            arrangement: Arrangement::NameValue("Melodics".to_string(), "Companion".to_string()),
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

    let mut enigo = Enigo::new(&Default::default()).unwrap();

    let mut app_state = AppState::default();

    loop {
        if !running.load(Ordering::SeqCst) {
            break; // Exit the loop if the termination signal is received
        }

        match rx.try_recv() {
            Ok(data) => {
                if let Ok(event) = MidiMessage::try_from(data.as_slice()) {
                    //input::log_midi_message(event.clone());
                    handle_button_presses(event, &mut enigo, &mut app_state, &mut launchkey_manager)
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

#[derive(Default)]
pub struct AppState {
    left_pads: Vec<Pad>,
    right_pads: Vec<Pad>,
}

fn handle_button_presses(
    message: MidiMessage,
    enigo: &mut Enigo,
    app_state: &mut AppState,
    mut launchkey_manager: &mut LaunchkeyManager<DAWMode>,
) {
    match message {
        MidiMessage::ControlChange(Channel::Ch1, control_function, value) => {
            match (
                LaunchKeyButton::from_value(control_function.0.into()),
                ButtonState::try_from(value),
            ) {
                (Some(LaunchKeyButton::Play), Ok(ButtonState::Pressed)) => {
                    enigo.key(enigo::Key::Space, Direction::Click).unwrap()
                }
                (Some(LaunchKeyButton::Record), Ok(ButtonState::Pressed)) => {
                    enigo.key(enigo::Key::Escape, Direction::Click).unwrap()
                }
                (Some(LaunchKeyButton::SceneLaunch), Ok(ButtonState::Pressed)) => {
                    enigo.key(enigo::Key::Return, Direction::Click).unwrap()
                }
                _ => {}
            }
        }

        MidiMessage::NoteOn(_, note, _) => {
            if let Some((pad, mode)) = Pad::from_value(note as u8) {
                match mode {
                    PadCCIndex::DAW => {
                        if app_state.left_pads.contains(&pad) {
                            app_state.left_pads.retain(|&x| x != pad);
                            app_state.right_pads.push(pad);

                            set_pad_color(&mut launchkey_manager, pad, CommonColor::NormalYellow);
                        } else if app_state.right_pads.contains(&pad) {
                            app_state.right_pads.retain(|&x| x != pad);

                            set_pad_color(&mut launchkey_manager, pad, CommonColor::Off);
                        } else {
                            app_state.left_pads.push(pad);

                            set_pad_color(&mut launchkey_manager, pad, CommonColor::NormalCyan);
                        }
                    }
                    PadCCIndex::Drum => {
                        // Change to custom color on press
                    }
                }
            }
        }

        _ => {}
    }
}

pub fn set_pad_color(
    launchkey_manager: &mut LaunchkeyManager<DAWMode>,
    pad: Pad,
    color: CommonColor,
) {
    launchkey_manager
        .send_commands(&[
            LaunchkeyCommand::SetPadColor {
                pad_in_mode: PadInMode::DAW(pad),
                mode: LEDMode::Flashing,
                color_palette_index: color.into(),
            },
            LaunchkeyCommand::SetPadColor {
                pad_in_mode: PadInMode::Drum(pad),
                mode: LEDMode::Stationary,
                color_palette_index: color.into(),
            },
        ])
        .unwrap();
}
