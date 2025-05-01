use crate::launchkey::bitmap::LaunchkeyBitmap;
use crate::launchkey::colors::{Color, ColorPaletteIndex};
use crate::launchkey::constants::{
    LaunchKeySku, BITMAP_HEADER_BYTE, BUTTON_BRIGHTNESS_OVERRIDE_CHANNEL, CC_CH7,
    CONFIGURE_DISPLAY_COMMAND, CUSTOM_COLOR_COMMAND, DISABLE_DRUM_DAW_MODE, ENABLE_DRUM_DAW_MODE,
    ENCODER_MODE_CC, FADER_MODE_CC, PAD_MODE_CC, SET_SCREEN_TEXT_COMMAND, SYSEX_TERMINATOR,
};
use crate::launchkey::modes::encoder_mode::EncoderMode;
use crate::launchkey::modes::fader_mode::FaderMode;
use crate::launchkey::modes::pad_mode::PadMode;
use crate::launchkey::surface::buttons::{Brightness, LaunchKeyButton};
use crate::launchkey::surface::display::{
    Arrangement, ContextualDisplayTarget, DisplayConfig, DisplayTarget, GlobalDisplayTarget,
};
use crate::launchkey::surface::pads::{LEDMode, PadInMode};
use std::ops::Deref;

/// Represents a high-level Launchkey command that can be converted into a MIDI message.
/// These are used to control pads, buttons, displays, and other device features.
#[derive(Debug, Clone)]
pub enum LaunchkeyCommand {
    SetDrumDAWMode(bool),
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
    SetScreenTextGlobal {
        target: GlobalDisplayTarget,
        arrangement: Arrangement,
    },
    SetScreenTextContextual {
        target: ContextualDisplayTarget,
        text: String,
    },
    SendScreenBitmap {
        target: GlobalDisplayTarget,
        bitmap: Box<LaunchkeyBitmap>,
    },
}

impl LaunchkeyCommand {
    /// Converts the [`LaunchkeyCommand`] into a MIDI byte sequence appropriate for the specific SKU.
    pub(crate) fn as_bytes(&self, sku: &LaunchKeySku) -> Vec<u8> {
        let header = sku.sys_ex_header();
        match self {
            LaunchkeyCommand::SetDrumDAWMode(enable) => {
                if *enable {
                    ENABLE_DRUM_DAW_MODE.into()
                } else {
                    DISABLE_DRUM_DAW_MODE.into()
                }
            }
            LaunchkeyCommand::SetPadMode(mode) => vec![CC_CH7, PAD_MODE_CC, mode.to_value()],
            LaunchkeyCommand::SetEncoderMode(mode) => {
                vec![CC_CH7, ENCODER_MODE_CC, mode.to_value()]
            }
            LaunchkeyCommand::SetFaderMode(mode) => vec![CC_CH7, FADER_MODE_CC, mode.to_value()],
            LaunchkeyCommand::SetButtonBrightness {
                launch_key_button,
                brightness,
            } => {
                vec![
                    BUTTON_BRIGHTNESS_OVERRIDE_CHANNEL,
                    (*launch_key_button).to_value(),
                    brightness.value(),
                ]
            }
            // Commands for pad LEDs
            LaunchkeyCommand::SetPadColor {
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
            LaunchkeyCommand::SetPadCustomColor { pad_in_mode, color } => {
                let mut data = header.to_vec();
                data.extend_from_slice(&[
                    CUSTOM_COLOR_COMMAND.0,
                    CUSTOM_COLOR_COMMAND.1,
                    (*pad_in_mode).to_index(),
                    color.r,
                    color.g,
                    color.b,
                ]);
                data.push(SYSEX_TERMINATOR);
                data
            }

            // Commands for screen control
            LaunchkeyCommand::SetScreenTextGlobal {
                target,
                arrangement,
            } => {
                let mut data: Vec<u8> = Vec::new();
                // Start with configuring the display for the arrangement
                data.extend(self.configure_display(
                    (*target).into(),
                    DisplayConfig::Arrangement(arrangement.clone()),
                    sku,
                ));

                // Handle different arrangement types
                match arrangement {
                    Arrangement::NameValue(name, value) => {
                        data.extend(self.set_screen_text(
                            (*target).into(),
                            0,
                            name.to_string(),
                            sku,
                        ));
                        data.extend(self.set_screen_text(
                            (*target).into(),
                            1,
                            value.to_string(),
                            sku,
                        ));
                    }
                    Arrangement::TitleNameValue(title, name, value) => {
                        data.extend(self.set_screen_text(
                            (*target).into(),
                            0,
                            title.to_string(),
                            sku,
                        ));
                        data.extend(self.set_screen_text(
                            (*target).into(),
                            1,
                            name.to_string(),
                            sku,
                        ));
                        data.extend(self.set_screen_text(
                            (*target).into(),
                            2,
                            value.to_string(),
                            sku,
                        ));
                    }
                    Arrangement::TitleEightNames(title, names) => {
                        data.extend(self.set_screen_text(
                            (*target).into(),
                            0,
                            title.to_string(),
                            sku,
                        ));
                        for (i, name) in names.iter().enumerate() {
                            data.extend(self.set_screen_text(
                                (*target).into(),
                                (i + 1) as u8,
                                name.to_string(),
                                sku,
                            ));
                        }
                    }
                    Arrangement::NameNumericValue(name) => {
                        data.extend(self.set_screen_text(
                            (*target).into(),
                            0,
                            name.to_string(),
                            sku,
                        ));
                    }
                }

                // Finalize with configuring the display for triggering
                data.extend(self.configure_display((*target).into(), DisplayConfig::Trigger, sku));

                data
            }
            LaunchkeyCommand::SetScreenTextContextual { target, text } => {
                self.set_screen_text((*target).into(), 0, text.to_string(), sku)
            }
            LaunchkeyCommand::SendScreenBitmap { target, bitmap } => {
                let mut data = header.to_vec();
                data.extend_from_slice(&[BITMAP_HEADER_BYTE, (*target).into()]);
                data.extend_from_slice(bitmap.deref().as_ref());
                data.push(SYSEX_TERMINATOR);
                data
            }
        }
    }

    /// Internal helper to create a SysEx message configuring the screen for a target and config.
    fn configure_display(
        &self,
        target: DisplayTarget,
        config: DisplayConfig,
        sku: &LaunchKeySku,
    ) -> Vec<u8> {
        let mut data = sku.sys_ex_header().to_vec();
        data.extend_from_slice(&[
            CONFIGURE_DISPLAY_COMMAND,
            target.into(),
            config.clone().into(),
        ]);
        data.push(SYSEX_TERMINATOR);
        data
    }

    /// Internal helper to create a SysEx message that sets text on a specific display field.
    fn set_screen_text(
        &self,
        target: DisplayTarget,
        field: u8,
        text: String,
        sku: &LaunchKeySku,
    ) -> Vec<u8> {
        let mut data = sku.sys_ex_header().to_vec();
        data.extend_from_slice(&[SET_SCREEN_TEXT_COMMAND, target.into(), field]);
        data.extend(text.as_bytes());
        data.push(SYSEX_TERMINATOR);
        data
    }
}