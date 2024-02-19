fn gen_notes() -> Vec<f64> {
    let a0: f64 = 27.50;
    let semitone = 2.0f64.powf(1.0 / 12.0);

    let mut tones = Vec::new();
    for note in 0..106 {
        tones.push(a0 * semitone.powf(note as f64));
    }
    tones
}

pub fn note_to_freq(note: &str) -> f64 {
    let letter: &str;
    let octave: &str;
    if note.len() == 2 {
        letter = &note[0..1];
        octave = &note[1..2];
    } else if note.len() == 3 {
        letter = &note[0..2];
        octave = &note[2..3];
    } else {
        panic!();
    }
    let note_value = match letter {
        "C" => 0,
        "C#" => 1,
        "D" => 2,
        "D#" => 3,
        "E" => 4,
        "F" => 5,
        "F#" => 6,
        "G" => 7,
        "G#" => 8,
        "A" => 9,
        "A#" => 10,
        "B" => 11,
        _ => panic!(),
    } + 12 * octave.parse::<usize>().unwrap();

    let notes = gen_notes();

    notes[note_value - 9] //We start at A0
}

pub fn freq_to_note(freq: f64) -> String {
    // special case of fit_to_scale using cromatic scale
    let scale = gen_notes();
    let scale_index = fit_to_scale(&scale, freq) + 9;
    let letters = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let letter = scale_index % 12;
    let octave = scale_index / 12;

    format!("{}{}", letters[letter], octave)
}

pub fn fit_to_scale(scale: &Vec<f64>, note_freq: f64) -> usize {
    // Return index of closest value using binary search
    // scale must be sorted
    let mut low = 0;
    let mut high = scale.len() - 1;
    for _ in 0..100 {
        let current_index = (low + high) / 2;
        let current = scale[current_index];

        // Check if search is over and select closest value
        if high - low <= 1 {
            if num::abs(note_freq - scale[low]) < num::abs(note_freq - scale[high]) {
                return low;
            }
            return high;
        }

        // Good ol' binary search
        if note_freq >= current {
            low = current_index;
        } else {
            high = current_index;
        }
    }
    panic!()
}

#[test]
fn test_a0() -> Result<(), String> {
    let notes = gen_notes();
    let note = note_to_freq("A0");
    assert_eq!(notes[0], note);
    Ok(())
}

#[test]
fn test_dsharp4() -> Result<(), String> {
    let notes = gen_notes();
    note_to_freq("D4");
    let note = note_to_freq("D#4");
    assert_eq!(notes[12 * 3 + 6], note);
    Ok(())
}

#[test]
fn test_440_to_a4() -> Result<(), String> {
    assert_eq!(freq_to_note(440.0), "A4");
    Ok(())
}

#[test]
fn test_415_to_gsharp4() -> Result<(), String> {
    assert_eq!(freq_to_note(415.304697579946), "G#4");
    Ok(())
}
