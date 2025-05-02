/// Defines bidirectional mapping methods for an enum, allowing conversion between
/// enum variants and corresponding values.
#[macro_export]
macro_rules! bidirectional_enum_mappings {
    ($name:ident, $type:ty, { $($variant:ident => $value:expr),* $(,)? }) => {
        impl $name {
            /// Returns the associated value for the enum variant.
            pub fn to_value(&self) -> $type {
                match self {
                    $(
                        $name::$variant => $value,
                    )*
                }
            }

            /// Returns the enum variant corresponding to the value, or `None` if invalid.
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

/// Defines bidirectional mapping methods for an enum with a custom mode.
/// This macro generates methods for converting between enum variants and values
/// for each specified mode.
#[macro_export]
macro_rules! bidirectional_enum_mappings_with_mode {
    // With custom mode enum name (mode is required)
    ($name:ident, $type:ty, $mode_name:ident, { $($mode:ident => {$($variant:ident => $value:expr),* $(,)?}),* $(,)? }) => {
        // Mode enum
        #[derive(Debug, Clone, Copy)]
        pub enum $mode_name {
            $($mode),*
        }

        impl $name {
            /// Converts the enum variant to its associated value based on the given mode.
            pub fn to_value_mode(&self, mode: $mode_name) -> $type {
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

            /// Converts a value back to its corresponding enum variant based on the mode.
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

            /// Returns the enum variant and its associated mode from a given value.
            pub fn from_value(value: $type) -> Option<($name, $mode_name)> {
                match value {
                    $(
                        $( $value => Some(($name::$variant, $mode_name::$mode)), )*
                    )*
                    _ => None,
                }
            }
        }
    };
}

/// Defines bidirectional mapping methods for an enum with an existing mode.
/// This macro generates methods for converting between enum variants and values
/// for each specified mode using an already defined mode enum.
#[macro_export]
macro_rules! bidirectional_enum_mappings_with_existing_mode {
    ($name:ident, $type:ty, $mode_name:ident, { $($mode:ident => { $($variant:ident => $value:expr),* $(,)? }),* $(,)? }) => {
        impl $name {
            /// Converts the enum variant to its associated value based on the given mode.
            pub fn to_value_mode(&self, mode: $mode_name) -> $type {
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

            /// Converts a value back to its corresponding enum variant based on the mode.
            pub fn from_value_mode(value: $type, mode: $mode_name) -> Option<$name> {
                match mode {
                    $(
                        $mode_name::$mode => Some(match value {
                            $(
                                $value => $name::$variant,
                            )*
                            _ => return None,
                        }),
                    )*
                }
            }

            /// Returns the enum variant and its associated mode from a given value.
            pub fn from_value(value: $type) -> Option<($name, $mode_name)> {
                match value {
                    $(
                        $(
                            $value => Some(($name::$variant, $mode_name::$mode)),
                        )*
                    )*
                    _ => None,
                }
            }
        }
    };
}