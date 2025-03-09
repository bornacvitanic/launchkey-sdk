// Macro for single bidirectional mappings.
#[macro_export]
macro_rules! bidirectional_enum_mappings {
    ($name:ident, $type:ty, { $($variant:ident => $value:expr),* $(,)? }) => {
        impl $name {
            // Generates O(1) lookup for single mapping
            pub fn to_value(self) -> $type {
                match self {
                    $(
                        $name::$variant => $value,
                    )*
                }
            }

            // Reverse lookup for single mapping (value -> enum variant)
            pub fn from_value(value: $type) -> Option<$name> {
                match value {
                    $(
                        $value => Some($name::$variant),
                    )*
                    _ => None,
                }
            }
        }
    };
}

// Macro for bidirectional mappings with a custom Mode enum.
#[macro_export]
macro_rules! bidirectional_enum_mappings_with_mode {
    // With custom mode enum name (mode is required)
    ($name:ident, $type:ty, $mode_name:ident, { $($mode:ident => {$($variant:ident => $value:expr),* $(,)?}),* $(,)? }) => {
        // Mode enum
        #[derive(Debug)]
        pub enum $mode_name {
            $($mode),*
        }

        impl $name {
            // Generates O(1) lookup for each mode
            pub fn to_value_mode(self, mode: $mode_name) -> $type {
                match mode {
                    $(
                        $mode_name::$mode => match self {
                            $(
                                $name::$variant => $value,
                            )*
                        },
                    )*
                }
            }

            // Reverse lookup for each mode (value -> enum variant)
            pub fn from_value_mode(value: $type, mode: $mode_name) -> Option<$name> {
                match mode {
                    $(
                        $mode_name::$mode => Some(match value {
                            $(
                                $value => $name::$variant,
                            )*
                            _ => return None
                        }),
                    )*
                }
            }
        }
    };
}
