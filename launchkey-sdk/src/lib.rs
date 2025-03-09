pub mod midi {
    pub mod input;
    pub mod to_hex;
}

pub mod launchkey {
    pub mod commands;
    pub mod manager;
    pub mod modes {
        pub mod encoder_mode;
        pub mod fader_mode;
        pub mod pad_mode;
    }
    pub mod constants;
    pub mod surface {
        pub mod buttons;
        pub mod display;
        pub mod encoders;
        pub mod faders;
        pub mod pads;
    }
    pub mod bitmap;
    pub mod colors;
}
