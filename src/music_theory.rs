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


