use crate::launchkey::colors::{Color, ColorPaletteIndex};
use crate::launchkey::constants::{
    LaunchKeySku, BUTTON_BRIGHTNESS_OVERRIDE_CHANNEL, DISABLE_DAW_MODE, ENABLE_DAW_MODE,
    SYSEX_TERMINATOR,
};
use crate::launchkey::modes::encoder_mode::EncoderMode;
use crate::launchkey::modes::fader_mode::FaderMode;
use crate::launchkey::modes::pad_mode::PadMode;
use crate::launchkey::surface::buttons::{Brightness, LaunchKeyButton};
use crate::launchkey::surface::display::{Arrangement, DisplayConfig, DisplayTarget};
use crate::launchkey::surface::pads::{LEDMode, PadInMode};

pub enum LaunchKeyCommand {
    EnableDAWMode,
    DisableDAWMode,
    SetPadMode(PadMode),
    SetEncoderMode(EncoderMode),
    SetFaderMode(FaderMode),
    SetButtonBrightness {
        launch_key_button: LaunchKeyButton,
        brightness: Brightness,
    },
    SetPadColor {
        pad_in_mode: PadInMode,
        mode: LEDMode,
        color_palette_index: ColorPaletteIndex,
    },
    SetPadCustomColor {
        pad_in_mode: PadInMode,
        color: Color,
    },
    ConfigureDisplay {
        target: DisplayTarget,
        config: DisplayConfig,
    },
    SetScreenTextArrangement {
        target: DisplayTarget,
        arrangement: Arrangement,
    },
    SetScreenText {
        target: DisplayTarget,
        field: u8,
        text: String,
    },
    SendScreenBitmap {
        target: DisplayTarget,
        bitmap_data: [u8; 1216],
    },
}

impl LaunchKeyCommand {
    pub(crate) fn as_bytes(&self, sku: &LaunchKeySku) -> Vec<u8> {
        let header = sku.sys_ex_header();
        match self {
            LaunchKeyCommand::EnableDAWMode => ENABLE_DAW_MODE.to_vec(),
            LaunchKeyCommand::DisableDAWMode => DISABLE_DAW_MODE.to_vec(),
            LaunchKeyCommand::SetPadMode(mode) => mode.as_bytes(),
            LaunchKeyCommand::SetEncoderMode(mode) => mode.as_bytes(),
            LaunchKeyCommand::SetFaderMode(mode) => mode.as_bytes(),
            LaunchKeyCommand::SetButtonBrightness {
                launch_key_button,
                brightness,
            } => {
                vec![
                    BUTTON_BRIGHTNESS_OVERRIDE_CHANNEL,
                    (*launch_key_button).to_index(),
                    brightness.value(),
                ]
            }
            // Commands for pad LEDs
            LaunchKeyCommand::SetPadColor {
                pad_in_mode,
                mode,
                color_palette_index,
            } => {
                let channel = mode.to_midi_channel(*pad_in_mode);
                vec![
                    channel,
                    (*pad_in_mode).to_index(),
                    color_palette_index.as_u8(),
                ]
            }
            LaunchKeyCommand::SetPadCustomColor { pad_in_mode, color } => {
                let mut data = header.to_vec();
                data.extend_from_slice(&[
                    0x01,
                    0x43,
                    (*pad_in_mode).to_index(),
                    color.r,
                    color.g,
                    color.b,
                ]);
                data.push(SYSEX_TERMINATOR);
                data
            }

            // Commands for screen control
            LaunchKeyCommand::ConfigureDisplay { target, config } => {
                let mut data = header.to_vec();
                data.extend_from_slice(&[0x04, (*target).into(), config.clone().into()]);
                data.push(SYSEX_TERMINATOR);
                data
            }
            LaunchKeyCommand::SetScreenText {
                target,
                field,
                text,
            } => {
                let mut data = header.to_vec();
                data.extend_from_slice(&[0x06, (*target).into(), *field]);
                data.extend(text.as_bytes());
                data.push(SYSEX_TERMINATOR);
                data
            }
            LaunchKeyCommand::SendScreenBitmap {
                target,
                bitmap_data,
            } => {
                let mut data = header.to_vec();
                data.extend_from_slice(&[0x09, (*target).into()]);
                data.extend_from_slice(bitmap_data);
                data.push(SYSEX_TERMINATOR);
                data
            }
            LaunchKeyCommand::SetScreenTextArrangement { target, arrangement } => {
                let mut data: Vec<u8> = Vec::new();
                // Start with configuring the display for the arrangement
                data.extend(LaunchKeyCommand::ConfigureDisplay { target: *target, config: DisplayConfig::Arrangement(arrangement.clone()) }.as_bytes(sku));

                // Handle different arrangement types
                match arrangement {
                    Arrangement::NameValue(name, value) => {
                        data.extend(LaunchKeyCommand::SetScreenText { target: *target, field: 0, text: name.to_string() }.as_bytes(sku));
                        data.extend(LaunchKeyCommand::SetScreenText { target: *target, field: 1, text: value.to_string() }.as_bytes(sku));
                    }
                    Arrangement::TitleNameValue(title, name, value) => {
                        data.extend(LaunchKeyCommand::SetScreenText { target: *target, field: 0, text: title.to_string() }.as_bytes(sku));
                        data.extend(LaunchKeyCommand::SetScreenText { target: *target, field: 1, text: name.to_string() }.as_bytes(sku));
                        data.extend(LaunchKeyCommand::SetScreenText { target: *target, field: 2, text: value.to_string() }.as_bytes(sku));
                    }
                    Arrangement::TitleEightNames(title, names) => {
                        data.extend(LaunchKeyCommand::SetScreenText { target: *target, field: 0, text: title.to_string() }.as_bytes(sku));
                        for (i, name) in names.iter().enumerate() {
                            data.extend(LaunchKeyCommand::SetScreenText { target: *target, field: (i + 1) as u8, text: name.to_string()}.as_bytes(sku));
                        }
                    }
                    Arrangement::NameNumericValue(name) => {
                        data.extend(LaunchKeyCommand::SetScreenText { target: *target, field: 0, text: name.to_string() }.as_bytes(sku));
                    }
                }
                
                // Finalize with configuring the display for triggering
                data.extend(LaunchKeyCommand::ConfigureDisplay { target: *target, config: DisplayConfig::Trigger }.as_bytes(sku));

                data
            }
        }
    }
}