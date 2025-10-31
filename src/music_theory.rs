// Music theory calculations for guitar fretboard

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

impl Key {
    pub fn from_int(value: i32) -> Key {
        match value % 12 {
            0 => Key::C,
            1 => Key::Cs,
            2 => Key::D,
            3 => Key::Ds,
            4 => Key::E,
            5 => Key::F,
            6 => Key::Fs,
            7 => Key::G,
            8 => Key::Gs,
            9 => Key::A,
            10 => Key::As,
            11 => Key::B,
            _ => Key::C,
        }
    }

    pub fn to_int(self) -> i32 {
        match self {
            Key::C => 0,
            Key::Cs => 1,
            Key::D => 2,
            Key::Ds => 3,
            Key::E => 4,
            Key::F => 5,
            Key::Fs => 6,
            Key::G => 7,
            Key::Gs => 8,
            Key::A => 9,
            Key::As => 10,
            Key::B => 11,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Key::C => "C",
            Key::Cs => "C#",
            Key::D => "D",
            Key::Ds => "D#",
            Key::E => "E",
            Key::F => "F",
            Key::Fs => "F#",
            Key::G => "G",
            Key::Gs => "G#",
            Key::A => "A",
            Key::As => "A#",
            Key::B => "B",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Note {
    pub note: Key,
    pub octave: i32,
}

impl Note {
    pub fn new(note: Key, octave: i32) -> Note {
        Note { note, octave }
    }

    pub fn name(self) -> String {
        format!("{}{}", self.note.name(), self.octave)
    }

    pub fn semitone_value(self) -> i32 {
        self.note.to_int() + (self.octave * 12)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scale {
    Major,
    NaturalMinor,
    MajorPentatonic,
    MinorPentatonic,
    MajorBlues,
    MinorBlues,
}

impl Scale {
    pub fn from_int(value: i32) -> Scale {
        match value {
            1 => Scale::Major,
            2 => Scale::NaturalMinor,
            3 => Scale::MajorPentatonic,
            4 => Scale::MinorPentatonic,
            5 => Scale::MajorBlues,
            6 => Scale::MinorBlues,
            _ => Scale::Major,
        }
    }

    pub fn to_int(self) -> i32 {
        match self {
            Scale::Major => 1,
            Scale::NaturalMinor => 2,
            Scale::MajorPentatonic => 3,
            Scale::MinorPentatonic => 4,
            Scale::MajorBlues => 5,
            Scale::MinorBlues => 6,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Scale::Major => "Major",
            Scale::NaturalMinor => "Natural Minor",
            Scale::MajorPentatonic => "Major Pentatonic",
            Scale::MinorPentatonic => "Minor Pentatonic",
            Scale::MajorBlues => "Major Blues",
            Scale::MinorBlues => "Minor Blues",
        }
    }

    // Returns semitone intervals from root for the scale
    fn intervals(self) -> &'static [i32] {
        match self {
            Scale::Major => &[0, 2, 4, 5, 7, 9, 11], // W-W-H-W-W-W-H
            Scale::NaturalMinor => &[0, 2, 3, 5, 7, 8, 10], // W-H-W-W-H-W-W
            Scale::MajorPentatonic => &[0, 2, 4, 7, 9],
            Scale::MinorPentatonic => &[0, 3, 5, 7, 10],
            Scale::MajorBlues => &[0, 3, 4, 7, 9],
            Scale::MinorBlues => &[0, 3, 5, 6, 7, 10],
        }
    }
}

// Standard guitar tuning (E2, A2, D3, G3, B3, E4)
// Returns base note for each string (6 strings, index 0 = low E)
// Use const fn to create static array
const fn make_base_notes() -> [Note; 6] {
    [
        Note { note: Key::E, octave: 2 }, // String 6 (low E)
        Note { note: Key::A, octave: 2 }, // String 5
        Note { note: Key::D, octave: 3 }, // String 4
        Note { note: Key::G, octave: 3 }, // String 3
        Note { note: Key::B, octave: 3 }, // String 2
        Note { note: Key::E, octave: 4 }, // String 1 (high E)
    ]
}

static BASE_NOTES: [Note; 6] = make_base_notes();

pub fn get_string_base_notes() -> &'static [Note; 6] {
    &BASE_NOTES
}

// Get the note at a specific string and fret position
// string: 0-5 (0 = low E, 5 = high E)
// fret: 0-23 (0 = open string)
pub fn get_note_at_position(string: u8, fret: u8) -> Note {
    let base = BASE_NOTES[string as usize];
    let semitones = base.note.to_int() + (base.octave * 12) + fret as i32;
    
    let note_value = semitones % 12;
    let octave = semitones / 12;
    
    Note::new(Key::from_int(note_value), octave)
}

// Get all notes in a scale for a given key
pub fn get_notes_in_scale(key: Key, scale: Scale) -> Vec<Note> {
    let intervals = scale.intervals();
    let mut notes = Vec::new();
    
    // Generate notes in multiple octaves for complete coverage
    for octave in 0..8 {
        for &interval in intervals {
            let semitone = key.to_int() + interval + (octave * 12);
            let note_value = semitone % 12;
            let note_octave = semitone / 12;
            notes.push(Note::new(Key::from_int(note_value), note_octave));
        }
    }
    
    notes
}

// Check if a note is in the given scale
// Optimized to avoid creating large vectors
pub fn is_note_in_scale(note: Note, key: Key, scale: Scale) -> bool {
    let intervals = scale.intervals();
    let key_offset = key.to_int();
    let note_value = note.note.to_int();
    
    // Check if the note's value matches any interval in the scale
    intervals.iter().any(|&interval| {
        (key_offset + interval) % 12 == note_value
    })
}

// Calculate frequency in Hz for a note using A4 = 440Hz standard tuning
pub fn calculate_frequency(note: Note) -> f32 {
    // A4 = 440Hz is at semitone 69 (MIDI standard)
    let a4_semitone = Note::new(Key::A, 4).semitone_value();
    let note_semitone = note.semitone_value();
    
    let semitones_above_a4 = note_semitone - a4_semitone;
    440.0 * 2.0_f32.powf(semitones_above_a4 as f32 / 12.0)
}

// Get fret positions that should have markers (dots)
pub fn get_marked_frets() -> Vec<u8> {
    vec![3, 5, 7, 9, 12, 15, 17, 19, 21]
}

// Check if a fret should have a marker dot
pub fn is_fret_marked(fret: u8) -> bool {
    get_marked_frets().contains(&fret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_from_int() {
        assert_eq!(Key::from_int(0), Key::C);
        assert_eq!(Key::from_int(11), Key::B);
        assert_eq!(Key::from_int(4), Key::E);
    }

    #[test]
    fn test_key_to_int() {
        assert_eq!(Key::C.to_int(), 0);
        assert_eq!(Key::B.to_int(), 11);
        assert_eq!(Key::E.to_int(), 4);
    }

    #[test]
    fn test_scale_intervals() {
        // Major scale: W-W-H-W-W-W-H (whole, whole, half, whole, whole, whole, half)
        let major = Scale::Major.intervals();
        assert_eq!(major.len(), 7);
        assert_eq!(major, vec![0, 2, 4, 5, 7, 9, 11]); // C D E F G A B
    }

    #[test]
    fn test_get_note_at_position() {
        // String 0 (low E), fret 0 = E2
        let note = get_note_at_position(0, 0);
        assert_eq!(note.note, Key::E);
        assert_eq!(note.octave, 2);

        // String 0, fret 5 = A2 (5 semitones up from E)
        let note = get_note_at_position(0, 5);
        assert_eq!(note.note, Key::A);
        assert_eq!(note.octave, 2);

        // String 5 (high E), fret 0 = E4
        let note = get_note_at_position(5, 0);
        assert_eq!(note.note, Key::E);
        assert_eq!(note.octave, 4);
    }

    #[test]
    fn test_is_note_in_scale() {
        // C Major scale contains: C, D, E, F, G, A, B
        let key = Key::C;
        let scale = Scale::Major;

        // Notes in C Major
        assert!(is_note_in_scale(Note { note: Key::C, octave: 4 }, key, scale));
        assert!(is_note_in_scale(Note { note: Key::D, octave: 4 }, key, scale));
        assert!(is_note_in_scale(Note { note: Key::E, octave: 4 }, key, scale));
        assert!(is_note_in_scale(Note { note: Key::F, octave: 4 }, key, scale));
        assert!(is_note_in_scale(Note { note: Key::G, octave: 4 }, key, scale));
        assert!(is_note_in_scale(Note { note: Key::A, octave: 4 }, key, scale));
        assert!(is_note_in_scale(Note { note: Key::B, octave: 4 }, key, scale));

        // Notes NOT in C Major (sharps/flats)
        assert!(!is_note_in_scale(Note { note: Key::Cs, octave: 4 }, key, scale));
        assert!(!is_note_in_scale(Note { note: Key::Ds, octave: 4 }, key, scale));
        assert!(!is_note_in_scale(Note { note: Key::Fs, octave: 4 }, key, scale));
    }

    #[test]
    fn test_calculate_frequency() {
        // A4 = 440 Hz (concert pitch)
        let a4 = Note { note: Key::A, octave: 4 };
        let freq = calculate_frequency(a4);
        assert!((freq - 440.0).abs() < 0.1); // Allow small floating point error

        // C4 = ~261.63 Hz
        let c4 = Note { note: Key::C, octave: 4 };
        let freq = calculate_frequency(c4);
        assert!((freq - 261.63).abs() < 0.5);
    }

    #[test]
    fn test_get_string_base_notes() {
        let base_notes = get_string_base_notes();
        assert_eq!(base_notes.len(), 6);
        // Standard guitar tuning: E2, A2, D3, G3, B3, E4
        assert_eq!(base_notes[0].note, Key::E);
        assert_eq!(base_notes[0].octave, 2);
        assert_eq!(base_notes[5].note, Key::E);
        assert_eq!(base_notes[5].octave, 4);
    }

    #[test]
    fn test_octave_wraparound() {
        // Test that going up 12 frets wraps around the octave
        let note1 = get_note_at_position(0, 0);  // E2 (string 0, open)
        let note2 = get_note_at_position(0, 12); // E3 (same note, octave up)
        assert_eq!(note1.note, note2.note);
        assert_eq!(note2.octave, note1.octave + 1);
    }

    #[test]
    fn test_fret_markers() {
        // Test marked frets
        assert!(is_fret_marked(3));
        assert!(is_fret_marked(12));
        assert!(is_fret_marked(21));
        
        // Test unmarked frets
        assert!(!is_fret_marked(1));
        assert!(!is_fret_marked(2));
        assert!(!is_fret_marked(4));
    }
}

