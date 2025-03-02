use wmidi::U7;

pub trait ToHexString {
    fn to_hex_string(&self) -> String;
}

impl ToHexString for &[U7] {
    fn to_hex_string(&self) -> String {
        self.iter()
            .map(|byte| format!("{:02X}", u8::from(*byte)))  // Convert U7 to u8 and format as hex
            .collect::<Vec<String>>()
            .join(" ")
    }
}