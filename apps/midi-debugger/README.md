# 🧪 MIDI Debugger

A command-line tool that connects to all available MIDI inputs and listens for events from a Novation Launchkey controller. It uses the [launchkey-sdk](https://crates.io/crates/launchkey-sdk) to visualize button presses, pad triggers, encoder twists, and display updates in real time.

## Features

- Logs detailed MIDI input events from your Launchkey.
- Sets up DAW mode with custom pad colors and screen text.
- Displays custom messages and bitmaps on the controller screen.
- Supports interpreting button/encoder/pad actions.
- Gracefully handles disconnects and Ctrl+C termination.

## Usage

```bash
cd apps/midi-debugger
cargo run --release
```

> Make sure your Launchkey device is connected and available as a MIDI input device on your system.

## Requirements

- Rust (edition 2021 or later)
- A connected Novation Launchkey device (MK4 supported)
- [`logo.png`](./logo.png) image file for screen rendering (optional, used for bitmap test)

## Output Example

```text
Listening for MIDI messages...
Launchkey Button: Play State: Pressed
Encoder Mode Change: Volume
Pad: Drum1, Mode: DAW
```

## Notes

- You can modify the `logo.png` file to test custom bitmap rendering on your controller's screen.
- MIDI input is handled using [midir](https://crates.io/crates/midir).
- All events are parsed using [`wmidi`](https://crates.io/crates/wmidi) and passed through `launchkey-sdk` abstractions.

## License

MIT — see [root LICENSE file](../../LICENSE.md).

---
_This is a demo tool for development/debugging. For SDK integration and production-level usage, refer to the main [`launchkey-sdk`](https://crates.io/crates/launchkey-sdk)._