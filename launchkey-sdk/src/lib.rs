pub mod midi {
    pub mod events;
    pub mod input;
}

pub mod launchkey {
    pub mod commands;
    pub mod manager;
    pub mod modes {
        pub mod pad_mode;
        pub mod encoder_mode;
        pub mod fader_mode;
    }
}