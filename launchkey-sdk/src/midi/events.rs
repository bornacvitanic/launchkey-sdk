#[derive(Debug)]
pub enum MidiEvent {
    NoteOn { note: u8, velocity: u8 },
    NoteOff { note: u8 },
    ControlChange { controller: u8, value: u8 },
    ProgramChange { program: u8 },
    SysEx { data: Vec<u8> },
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
            0xB0..=0xBF => {
                // Control Change
                Some(MidiEvent::ControlChange {
                    controller: message[1],
                    value: message[2],
                })
            }
            0xC0..=0xCF => {
                // Program Change
                if message.len() < 2 {
                    return None; // Invalid message length
                }
                Some(MidiEvent::ProgramChange {
                    program: message[1],
                })
            }
            0xF0 => {
                // SysEx
                if message.len() < 2 || *message.last().unwrap_or(&0) != 0xF7 {
                    return None; // Invalid SysEx message
                }
                Some(MidiEvent::SysEx {
                    data: message[1..message.len() - 1].to_vec(),
                })
            }
            _ => None, // Unknown event
        }
    }
}
