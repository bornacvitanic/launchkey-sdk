[![Test](https://github.com/bornacvitanic/launchkey-sdk/actions/workflows/rust.yml/badge.svg)](https://github.com/bornacvitanic/launchkey-sdk/actions/workflows/rust.yml)
[![dependency status](https://deps.rs/repo/github/bornacvitanic/launchkey-sdk/status.svg?path=launchkey-sdk)](https://deps.rs/repo/github/bornacvitanic/launchkey-sdk?path=launchkey-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/launchkey-sdk.svg)](https://crates.io/crates/launchkey-sdk)
[![Download](https://img.shields.io/badge/download-releases-blue.svg)](https://github.com/bornacvitanic/launchkey-sdk/releases)
[![Docs.rs](https://img.shields.io/docsrs/launchkey-sdk)](https://docs.rs/launchkey-sdk)

![Novation Launchkey](https://www.rockitdistribution.com/wp-content/uploads/2025/02/LAUNCHKEY-MK4-full-range-web-banner.png)

# Launchkey SDK

`launchkey-sdk` is a type-safe Rust SDK for **Novation Launchkey** MIDI controllers.\
Novation provides a detailed programmer's guide for their MIDI protocol, but no official library exists — this SDK fills that gap with a complete, ergonomic, and extensible solution in Rust.

It provides complete programmatic control over **pads**, **encoders**, **faders**, **screen(s)**, and more. Whether you're building custom DAW integrations, stage tools, or creative MIDI workflows, this SDK gives you low-level access with high-level ergonomics.

The SDK is designed to support both Standalone and DAW mode operations, includes RGB color and bitmap display support, and is built for portability across platforms.

### ⚠️ Disclaimer & Technical Reference

> This project is an **unofficial**, **independently developed** SDK for Novation Launchkey MIDI controllers.  
> It is **not affiliated with, endorsed by, or supported by Novation or Focusrite**.

The SDK is based on the official [Launchkey MK4 Programmer’s Reference Guide (PDF)](https://fael-downloads-prod.focusrite.com/customer/prod/downloads/launchkey_mk4_programmer_s_reference_guide_v2_en.pdf) provided by Novation.  
It maps all known SysEx, CC, and control behavior as documented in that guide.


## Features

### 🎛️ Control Surfaces
- Change encoder, fader, and pad modes
- DAW/Drum mode toggling
- Set pad colors (stationary, flashing, pulsing)
- Customize button brightness
- Send raw or structured MIDI commands

### 🖼️ Display Output
- Send 128x64 monochrome bitmaps to the onboard screen
- Write multi-line text with predefined arrangements
- Contextual and global display targets

### 🌈 Color Utilities
- Use predefined `CommonColor` variants
- Convert full-range (0–255) or palette-range (97–255) RGB values to 0–127
- Create and map RGB values to palette indices

---

## Supported Devices

| Model             | Regular  | Mini     |
|-------------------|----------|----------|
| Launchkey MK4     | ✅       | ✅       |
| Launchkey MK3     | ❌       | ❌       |
| Launchkey MK2     | ❌       | ❌       |

---

## Roadmap

- [ ] **Launchkey MK4 Feature Control Support**  
  Implement support for Launchkey MK4's full MIDI CC-based feature control system over DAW In port:
  - [ ] Channel 7: Send control commands
  - [ ] Channel 8: Send queries for current feature state
  - [ ] Listen for reply messages on Channel 7 confirming state or response
  - [ ] Implement control for major feature groups:
    - [ ] Arp settings: Swing, Deviate, Ties, Accents, Ratchets, Type, Rate, Octave, Gate, Latch, etc.
    - [ ] Scale and note layout: Behaviour, Root note, Mode, Shift toggle
    - [ ] DAW interaction toggles: Performance Note Redirect, Touch Events, Encoder Relativity, Fader Pickup
    - [x] Pad/Encoder/Fader layout selection
    - [ ] Pad/Keybed velocity, aftertouch config, fixed velocity control
    - [x] LED brightness
    - [ ] Screen brightness and timeouts
    - [ ] Chord Map settings and MIDI routing
    - [ ] Arp connection and split zone selection
    - [ ] Custom mode defaults and pick-up behaviors
  - [ ] Expose a high-level `FeatureControl` API for reading/writing values by logical name

- [ ] **Launchkey MK3 Support**  
  Add support for older MK3 devices with their unique SysEx and CC mappings.

- [ ] **Launchkey MK2 Support**  
  Add support for older MK2 devices with their unique SysEx and CC mappings.

- [ ] **Bitmap Composition Utilities**  
  Add text/image/sprite rendering support into `LaunchkeyBitmap`, including:
    - Font rasterization
    - Shape drawing (lines, boxes, etc.)
    - Icon compositing and asset embedding

- [ ] **Custom Pad Layout Mapping**  
  User-defined pad banks and aliases for dynamic page switching and alternate grid mappings.

- [ ] **Preset Management API**  
  Load/save pad layouts, colors, fader modes, and display state as named user presets.

- [ ] **Remember Active Pad/Encoder/Fader Mode State**  
  Implement functionality to remember the state of the active pad, encoder, and fader mode, even across session resets.

- [ ] **Support for Multiple/Virtual Modes**  
  Enable support for multiple or virtual modes, including:
    - Infinite encoder banks for expanded control options.
    - Multiple pad sub-modes for deeper interaction, such as different layers or sets of controls within one physical mode.

- [ ] **CLI Tool (`launchkey-cli`)**  
  A separate command-line interface for sending test commands, bitmaps, display updates, etc.

- [ ] **Auto Hotplug Detection**  
  Automatically reconnect or react to device plug/unplug events on supported platforms.

### Example Gallery

- [x] **MIDI Monitor**  
  Log all incoming MIDI messages, highlighting Launchkey-specific controls like encoders, faders, and pads.

- [ ] **Visual Metronome**  
  Flash a pad or row of pads in time with a given BPM.

- [ ] **Interactive Finger Drumming Pad Trainer**  
  Create an interactive pad trainer to help users practice finger drumming, with real-time feedback and customizable exercises.

- [ ] **Custom Drum Pad Layout**  
  Re-map pads to custom MIDI note triggers or DAW controls, with LED and screen feedback for each pad.

- [ ] **Performance Mode**  
  Animate pads with pulse/flashing color patterns and update display labels dynamically during performance.

- [ ] **Live Parameter Visualizer**  
  Show encoder or fader values visually on the screen — as bars, dials, or numbers — while they are being used.

## Cross-platform
This library works on **Windows**, **macOS**, and **Linux** without any external dependencies.

## Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
launchkey-sdk = "0.1.0"
```

## Getting Started

### 1. **Initialize Launchkey Manager and Set DAW Mode**
```rust
fn main() {
  let mut manager = LaunchkeyManager::default().unwrap();  
  let mut manager = manager.into_daw_mode().unwrap();
}
```

### 2. **Set Pad Color**
```rust
fn main() {
  //...
  manager.send_command(LaunchkeyCommand::SetPadColor {
      pad_in_mode: PadInMode::DAW(Pad::Mixer),
      mode: LEDMode::Stationary,
      color_palette_index: CommonColor::BrightGreen.into(),
  }).unwrap();
}
```

### 3. **Set Encoder Mode**
```rust
fn main() {
  //...
  manager.send_command(LaunchkeyCommand::SetEncoderMode {
      mode: EncoderMode::Normal,
  }).unwrap();
}
```

### 4. **Set Screen Text**
```rust
fn main() {
  //...
  manager.send_command(LaunchkeyCommand::SetScreenTextGlobal {
      target: GlobalDisplayTarget::Stationary,
      arrangement: Arrangement::NameValue("DAW Mode".to_string(), "Ready".to_string()),
  }).unwrap();
}
```

### 5. **React to MIDI Launchkey interactions**
```rust
fn main() {
  let (tx, rx) = mpsc::channel();
  let midi_in = MidiInput::new("MIDI Listener").unwrap();
  let ports = get_named_midi_ports(&midi_in);

  // Connect to all available MIDI ports
  let _connections = connect_all_midi_ports(&ports, tx.clone());
  
  loop {
    if let Ok(data) = rx.recv() {
      if let Ok(event) = MidiMessage::try_from(data.as_slice()) {
        match event {
          // Recognize Pad Press
          MidiMessage::NoteOn(_, note, _) => {
            if let Some((pad, mode)) = Pad::from_value(note as u8) {
              println!("Pad {:?} pressed in mode {:?}", pad, mode);
            }
          }

          // Recognize Pad Mode Change
          MidiMessage::ControlChange(_, control_function, value)
          if control_function.equals_u8(PAD_MODE_CC) =>
            {
              println!("Pad Mode Changed: {:?}", PadMode::from_value(value.into()));
            }

          _ => {}
        }
      }
    }
  }
}
```


## API Highlights
- `LaunchkeyManager`: Handles MIDI communication and device state
- `Commands`: Type-safe API for all device operations (pads, displays, modes)
- `Colors`: Both predefined palette indices and custom RGB values
- `MIDI Handling`: Built-in utilities for control change, note on/off, SysEx messages

## Example Applications

This repository includes additional tools built on top of the SDK, located in the [`/apps`](./apps) directory:

| Name                             | Description                                                                                       |
|----------------------------------|---------------------------------------------------------------------------------------------------|
| [`midi-debugger`](./apps/midi-debugger)           | A real-time MIDI event logger to inspect and debug Launchkey messages.                            |
| [`melodics-companion`](./apps/melodics-companion) | A helper tool to augment Melodics training sessions with additional controls and visual feedback. |

Each tool can be run independently and includes its own README for usage and setup instructions.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

## Acknowledgments
- [midir](https://crates.io/crates/midir) - MIDI communication library for Rust, providing a simple interface for MIDI input and output.
- [wmidi](https://crates.io/crates/wmidi) - A crate for handling MIDI messages in a platform-agnostic manner.
- [image](https://crates.io/crates/image) - A library for image processing, including bitmap conversion and manipulation.
- [strum](https://crates.io/crates/strum) - A set of utilities for working with enums in Rust, including deriving methods and iterators.

## Contact

- **Email**: [borna.cvitanic@gmail.com](mailto:borna.cvitanic@gmail.com)
- **GitHub Issues**: [GitHub Issues Page](https://github.com/bornacvitanic/launchkey-sdk/issues)