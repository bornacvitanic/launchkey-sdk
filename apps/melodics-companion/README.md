# 🎹 Melodics Companion

An automation utility that turns your Novation Launchkey controller into a streamlined companion for [Melodics](https://melodics.com/). It enables quick pad selection with color feedback and provides mapped shortcut actions using your controller's transport buttons.

## Features

- Press pads to toggle state with visual LED feedback.
- Split selected pads into left/right groups.
- Transport buttons mapped to Melodics shortcuts:
    - **Play** → `Space` (Start/Stop)
    - **Record** → `Esc` (Reset/Exit)
    - **Scene Launch** → `Return` (Confirm/Next)
- Display custom DAW name and static banner on controller screen.

## Usage

```bash
cd apps/melodics-companion
cargo run --release
```

Ensure your Launchkey is connected before launching.

## Requirements

- Rust (edition 2021 or later)
- Launchkey MK4 connected and supported
- [launchkey-sdk](https://crates.io/crates/launchkey-sdk)
- [`enigo`](https://crates.io/crates/enigo) for simulating keyboard input

## Behavior

In DAW Pad Mode:
- Press a pad once → lights up **cyan** (left group)
- Press it again → moves to **yellow** (right group)
- Press it a third time → turns **off**

Pad states are visually represented using flashing while in configuration DAW mode and stationary while in drum mode

## Configuration

- You can customize the mapped keys and colors in `handle_button_presses` and `set_pad_color`.
- The static banner text and DAW label can also be adjusted.

## License

see [LICENSE](../../LICENSE.md) in root.

---
_This tool is meant as a personal productivity and learning aid while using Melodics._