use wmidi::U7;

/// Trait for converting byte slices into formatted hexadecimal strings.
pub trait ToHexString {
    /// Converts to uppercase space-separated hex string (e.g., "F0 7E 00").
    fn to_hex_string(&self) -> String;
}

impl ToHexString for &[U7] {
    fn to_hex_string(&self) -> String {
        self.iter()
            .map(|byte| format!("{:02X}", u8::from(*byte))) // Convert U7 to u8 and format as hex
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl ToHexString for &[u8] {
    fn to_hex_string(&self) -> String {
        self.iter()
            .map(|byte| format!("{:02X}", byte))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl ToHexString for Vec<u8> {
    fn to_hex_string(&self) -> String {
        self.as_slice().to_hex_string()
    }
}