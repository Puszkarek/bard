use crate::models::{LyricLine, LyricsStatus};

const POSITION_OFFSET_SECONDS: f64 = 1.0;

pub fn get_lyrics_status(lyrics: &[LyricLine], position: f64) -> LyricsStatus {
    // Include offset in position comparison
    let adjusted_position = position + POSITION_OFFSET_SECONDS;

    let current_index = lyrics
        .iter()
        .enumerate()
        .take_while(|(_, line)| line.timestamp <= adjusted_position)
        .map(|(i, _)| i)
        .last();

    match current_index {
        Some(i) => {
            let current_line = &lyrics[i].text;

            // Check if there's a next line
            if i < lyrics.len() - 1 {
                let next_line = &lyrics[i + 1].text;
                return LyricsStatus {
                    current_line: current_line.to_string(),
                    next_line: next_line.to_string(),
                    next_timestamp: Some(lyrics[i + 1].timestamp),
                };
            } else {
                return LyricsStatus {
                    current_line: current_line.to_string(),
                    next_line: String::new(),
                    next_timestamp: None,
                };
            }
        }
        None => {
            // No current line found, check if there's an upcoming line
            // The user is probably in the intro, so we'll return the first lyric as the next one
            if !lyrics.is_empty() {
                // Get the first lyric as the next one

                return LyricsStatus {
                    current_line: String::new(),
                    next_line: lyrics[0].text.to_string(),
                    next_timestamp: Some(lyrics[0].timestamp),
                };
            }

            return LyricsStatus {
                current_line: String::new(),
                next_line: String::new(),
                next_timestamp: None,
            };
        }
    }
}

pub fn format_lyrics_for_tooltip(lyrics: &[LyricLine]) -> String {
    lyrics
        .iter()
        .map(|line| line.text.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}
