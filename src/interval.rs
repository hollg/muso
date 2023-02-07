#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IntervalQuality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IntervalSize {
    Unison,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Octave,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval {
    pub size: IntervalSize,
    pub quality: IntervalQuality,
    pub semitones: u8,
}

impl Interval {
    pub fn new(size: IntervalSize, quality: IntervalQuality) -> Self {
        let semitones = match (size, quality) {
            (IntervalSize::Unison, IntervalQuality::Perfect) => 0,
            (IntervalSize::Second, IntervalQuality::Minor) => 1,
            (IntervalSize::Second, IntervalQuality::Major) => 2,
            (IntervalSize::Third, IntervalQuality::Minor) => 3,
            (IntervalSize::Third, IntervalQuality::Major) => 4,
            (IntervalSize::Fourth, IntervalQuality::Perfect) => 5,
            (IntervalSize::Fifth, IntervalQuality::Diminished) => 6,
            (IntervalSize::Fifth, IntervalQuality::Perfect) => 7,
            (IntervalSize::Sixth, IntervalQuality::Minor) => 8,
            (IntervalSize::Sixth, IntervalQuality::Major) => 9,
            (IntervalSize::Seventh, IntervalQuality::Minor) => 10,
            (IntervalSize::Seventh, IntervalQuality::Major) => 11,
            (IntervalSize::Octave, IntervalQuality::Perfect) => 12,
            _ => panic!("Invalid interval size"),
        };

        Interval {
            size,
            quality,
            semitones,
        }
    }
    pub fn from_semitones(num_semitones: u8) -> Self {
        match num_semitones {
            0 => Interval {
                size: IntervalSize::Unison,
                quality: IntervalQuality::Perfect,
                semitones: 0,
            },
            1 => Interval {
                size: IntervalSize::Second,
                quality: IntervalQuality::Minor,
                semitones: 1,
            },
            2 => Interval {
                size: IntervalSize::Second,
                quality: IntervalQuality::Major,
                semitones: 2,
            },
            3 => Interval {
                size: IntervalSize::Third,
                quality: IntervalQuality::Minor,
                semitones: 3,
            },
            4 => Interval {
                size: IntervalSize::Third,
                quality: IntervalQuality::Major,
                semitones: 4,
            },
            5 => Interval {
                size: IntervalSize::Fourth,
                quality: IntervalQuality::Perfect,
                semitones: 5,
            },
            6 => Interval {
                size: IntervalSize::Fifth,
                quality: IntervalQuality::Diminished,
                semitones: 6,
            },
            7 => Interval {
                size: IntervalSize::Fifth,
                quality: IntervalQuality::Perfect,
                semitones: 7,
            },
            8 => Interval {
                size: IntervalSize::Sixth,
                quality: IntervalQuality::Minor,
                semitones: 8,
            },
            9 => Interval {
                size: IntervalSize::Sixth,
                quality: IntervalQuality::Major,
                semitones: 9,
            },
            10 => Interval {
                size: IntervalSize::Seventh,
                quality: IntervalQuality::Minor,
                semitones: 10,
            },
            11 => Interval {
                size: IntervalSize::Seventh,
                quality: IntervalQuality::Major,
                semitones: 11,
            },
            12 => Interval {
                size: IntervalSize::Octave,
                quality: IntervalQuality::Perfect,
                semitones: 12,
            },
            _ => panic!("Invalid interval size"),
        }
    }

    pub fn to_semitones(&self) -> u8 {
        match self.size {
            IntervalSize::Unison => 0,
            IntervalSize::Second => match self.quality {
                IntervalQuality::Minor => 1,
                IntervalQuality::Major => 2,
                _ => panic!("Invalid interval quality"),
            },
            IntervalSize::Third => match self.quality {
                IntervalQuality::Minor => 3,
                IntervalQuality::Major => 4,
                _ => panic!("Invalid interval quality"),
            },
            IntervalSize::Fourth => match self.quality {
                IntervalQuality::Perfect => 5,
                _ => panic!("Invalid interval quality"),
            },
            IntervalSize::Fifth => match self.quality {
                IntervalQuality::Diminished => 6,
                IntervalQuality::Perfect => 7,
                _ => panic!("Invalid interval quality"),
            },
            IntervalSize::Sixth => match self.quality {
                IntervalQuality::Minor => 8,
                IntervalQuality::Major => 9,
                _ => panic!("Invalid interval quality"),
            },
            IntervalSize::Seventh => match self.quality {
                IntervalQuality::Minor => 10,
                IntervalQuality::Major => 11,
                _ => panic!("Invalid interval quality"),
            },
            IntervalSize::Octave => 12,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_interval_from_semitones() {
        let interval = Interval::from_semitones(2);
        assert_eq!(interval.size, IntervalSize::Second);
        assert_eq!(interval.quality, IntervalQuality::Major);

        let interval = Interval::from_semitones(6);
        assert_eq!(interval.size, IntervalSize::Fifth);
        assert_eq!(interval.quality, IntervalQuality::Diminished);
    }

    #[test]
    fn converts_interval_to_semitones() {
        let interval = Interval::new(IntervalSize::Second, IntervalQuality::Major);
        assert_eq!(interval.to_semitones(), 2);

        let interval = Interval::new(IntervalSize::Fifth, IntervalQuality::Diminished);
        assert_eq!(interval.to_semitones(), 6);
    }
}
