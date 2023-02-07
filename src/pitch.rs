use crate::Interval;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum Accidental {
    Flat,
    Natural,
    Sharp,
}

/// Represents a pitch, e.g. C, A♭, F♯
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pitch {
    letter: Letter,
    accidental: Option<Accidental>,
}

impl Pitch {
    pub fn from_string(string: String) -> Self {
        let letter = match string.chars().next().unwrap() {
            'A' => Letter::A,
            'B' => Letter::B,
            'C' => Letter::C,
            'D' => Letter::D,
            'E' => Letter::E,
            'F' => Letter::F,
            'G' => Letter::G,
            _ => panic!("Invalid note name"),
        };

        let accidental = match string.chars().nth(1) {
            Some('b') => Some(Accidental::Flat),
            Some('♭') => Some(Accidental::Flat),
            Some('#') => Some(Accidental::Sharp),
            Some('♯') => Some(Accidental::Sharp),
            _ => None,
        };

        Pitch { letter, accidental }
    }

    pub fn from_interval(pitch: Pitch, interval: Interval) -> Self {
        Self::from_semitones(pitch, interval.semitones)
    }

    fn from_semitones(pitch: Pitch, semitones: u8) -> Self {
        let pitch_index = PITCHES.iter().position(|&p| p == pitch).unwrap();
        let new_pitch_index = (pitch_index + semitones as usize) % PITCHES.len();

        PITCHES[new_pitch_index]
    }
}

pub const PITCHES: [Pitch; 12] = [
    Pitch {
        letter: Letter::A,
        accidental: None,
    },
    Pitch {
        letter: Letter::B,
        accidental: Some(Accidental::Flat),
    },
    Pitch {
        letter: Letter::B,
        accidental: None,
    },
    Pitch {
        letter: Letter::C,
        accidental: None,
    },
    Pitch {
        letter: Letter::D,
        accidental: Some(Accidental::Flat),
    },
    Pitch {
        letter: Letter::D,
        accidental: None,
    },
    Pitch {
        letter: Letter::E,
        accidental: Some(Accidental::Flat),
    },
    Pitch {
        letter: Letter::E,
        accidental: None,
    },
    Pitch {
        letter: Letter::F,
        accidental: None,
    },
    Pitch {
        letter: Letter::G,
        accidental: Some(Accidental::Flat),
    },
    Pitch {
        letter: Letter::G,
        accidental: None,
    },
    Pitch {
        letter: Letter::A,
        accidental: Some(Accidental::Flat),
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_pitch_from_string() {
        let pitch = Pitch::from_string("A".to_string());
        assert_eq!(pitch.letter, Letter::A);
        assert_eq!(pitch.accidental, None);

        let pitch = Pitch::from_string("Cb".to_string());
        assert_eq!(pitch.letter, Letter::C);
        assert_eq!(pitch.accidental, Some(Accidental::Flat));

        let pitch = Pitch::from_string("F#".to_string());
        assert_eq!(pitch.letter, Letter::F);
        assert_eq!(pitch.accidental, Some(Accidental::Sharp));
    }

    #[test]
    fn builds_pitch_from_interval() {
        let pitch = Pitch::from_string("C".to_string());
        let interval = Interval::from_semitones(4);
        let new_pitch = Pitch::from_interval(pitch, interval);
        assert_eq!(new_pitch, Pitch::from_string("E".to_string()));

        let pitch = Pitch::from_string("G".to_string());
        let interval = Interval::from_semitones(3);
        let new_pitch = Pitch::from_interval(pitch, interval);
        assert_eq!(new_pitch, Pitch::from_string("Bb".to_string()));
    }

    #[test]
    fn builds_pitch_from_semitones() {
        let pitch = Pitch::from_string("C".to_string());
        let new_pitch = Pitch::from_semitones(pitch, 4);
        assert_eq!(new_pitch, Pitch::from_string("E".to_string()));

        let pitch = Pitch::from_string("G".to_string());
        let new_pitch = Pitch::from_semitones(pitch, 3);
        assert_eq!(new_pitch, Pitch::from_string("Bb".to_string()));
    }
}
