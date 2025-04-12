use crate::models::{LyricLine, LyricsStatus};

pub fn get_lyrics_status(lyrics: &[LyricLine], position: f64) -> LyricsStatus {
    // Find the lyric line that corresponds to the current position
    let mut current_line = "";
    let mut next_line = "";

    for i in 0..lyrics.len() {
        let line = &lyrics[i];
        if line.timestamp <= position {
            current_line = line.text.as_str();
            if i < lyrics.len() - 1 {
                next_line = lyrics[i + 1].text.as_str();
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
    }
}

pub fn format_lyrics_for_tooltip(lyrics: &[LyricLine]) -> String {
    lyrics
        .iter()
        .map(|line| {
            if line.timestamp > 0.0 {
                let minutes = (line.timestamp / 60.0).floor() as i32;
                let seconds = (line.timestamp % 60.0).floor() as i32;
                format!("[{:02}:{:02}] {}", minutes, seconds, line.text)
            } else {
                line.text.clone()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}
