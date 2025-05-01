# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2025-05-01

### Bug Fixes

- Fix pads.rs PadCustom4 having wrong index value

- Fix manager.rs having flipped Drum DAW Mode setting values


### Documentation

- Add README.md

- Update scripts to add documentation

- Add LICENSE.md

- Update manager.rs to add docstrings


### Features

- Update Cargo.toml to add more info

- Add github workflow

- Update to_hex.rs to add more implementations

- Update input.rs to add connect_all_midi_ports method

- Update melodics-companion main.rs to add support for custom pad color changing mode

- Add melodics-companion workspace member

- Update buttons.rs to add bidirectional mapping for button values and ButtonState enum

- Add cliff.toml for changelog generation support

- Update main.rs to add method for interpreting launchkey midi messages

- Update macros.rs to add bidirectional_enum_mappings_with_existing_mode

- Update commands.rs to add a command for entering drum daw mode

- Update macros.rs to add a from_value method for when the mode is unknown

- Update constants.rs to add more named constants

- Update pad_mode to add DrumDAW mode so that later it can be gotten by value when mapped bidirectionally

- Add macros.rs to add macros for bidirectional bindings/mappings of enum variants and values

- Add bitmap.rs to add Launchkey Bitmap support from images

- Update manager.rs to add bitmap support for standalong mode

- Update colors.rs to add orange common color support

- Update manager.rs to implement type state pattern for DAW mode exclusive commands

- Add RustRover meta files

- Add Global and Contextual display targets

- Update colors.rs to implement From traits for ColorPaletteIndex and Color

- Update commands.rs to add SetScreenTextArrangement command

- Update manager.rs to add send_commands method

- Add wmidi crate

- Add Arrangement enum

- Add ModeNameTarget enum

- Add encoders.rs to specify encoder indexes

- Add colors module with support for full launchkey color palette indexes, predefined colors and custom colors

- Update commands to add SetButtonBrightness command

- Update commands.rs to add Pad enum contaning DAW and Drum variants for type safe pad coloring

- Add SysEx support, Launckey SKU specification support, new MidiEvent type support


### Moves

- Move Buttons and Pads to separate modules

- Move Constants and Display related stuff to separate modules

- Move modes to separate files


### Refactors

- Refactor file susing cargo fmt

- Refactor main.rs to clean up for iteration to a more functional style

- Refactor main.rs for melodics companion to clean up some match statements

- Refactor files using clippy

- Refactor files using cargo fmt

- Refactor files using cargo fmt


### Removals

- Remove methods for manually senabling or disabling drum daw mode


### Renames

- Update commands.rs to rename text setting command


### Revert

- Revert "Remove methods for manually senabling or disabling drum daw mode"

This reverts commit de1b4a41ed5ea739eecd565c2e89eee05acb3aee.


### Styling

- Update main.rs to clean up input port listening logic


### Updates

- Update main.rs for melodics-companion to auto derive AppState default

- Update input.rs to handle more MidiMessage types

- Update commands.rs to box the bitmap in the SendScreenBitmap command to reduce LaunchkeyCommand variant size difference

- Update input.rs to improve logging

- Update modes to utilise bidirection enum mappings

- Update surface element modes to derive Clone

- Update display.rs to make Temporary DisplayTarget take a TemporaryTarget as argument

- Update main.rs to auto connect to all MidiInputs

- Update commands to support setting of both DAW pad mode and Drum pad mode pad colors

- Update commands to extend PadModes, EncoderModes and FaderModes