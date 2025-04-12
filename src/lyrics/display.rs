use crate::models::{LyricLine, LyricsStatus};

pub fn get_lyrics_status(lyrics: &[LyricLine], position: f64) -> LyricsStatus {
    // Find the lyric line that corresponds to the current position
    let mut current_line = "";
    let mut next_line = "";
    let mut next_timestamp = None;

    for i in 0..lyrics.len() {
        let line = &lyrics[i];
        if line.timestamp <= position {
            current_line = line.text.as_str();
            if i < lyrics.len() - 1 {
                let next_lyric_line = &lyrics[i + 1];
                next_line = next_lyric_line.text.as_str();
                next_timestamp = Some(next_lyric_line.timestamp);
            } else {
                next_line = "";
            }
        } else {
            break;
        }
    }

    LyricsStatus {
        current_line: current_line.to_string(),
        next_line: next_line.to_string(),
        next_timestamp,
    }
}

pub fn format_lyrics_for_tooltip(lyrics: &[LyricLine]) -> String {
    lyrics
        .iter()
        .map(|line| line.text.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}
