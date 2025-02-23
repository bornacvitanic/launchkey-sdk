#[derive(Debug)]
pub enum MidiEvent {
    NoteOn { note: u8, velocity: u8 },
    NoteOff { note: u8 },
}

impl MidiEvent {
    pub fn parse(message: &[u8]) -> Option<MidiEvent> {
        if message.len() < 3 {
            return None; // Invalid message
        }
        let status = message[0];
        let note = message[1];
        let velocity = message[2];
        match status {
            144 if velocity > 0 => Some(MidiEvent::NoteOn { note, velocity }),
            128 | 144 if velocity == 0 => Some(MidiEvent::NoteOff { note }),
            _ => None, // Unknown event
        }
    }
}
