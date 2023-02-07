use crate::pitch::Pitch;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Note {
    pitch: Pitch,
    octave: u8,
}
