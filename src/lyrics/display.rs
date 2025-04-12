use crate::models::LyricLine;

pub fn get_current_lyric(lyrics: &[LyricLine], position: f64) -> LyricLine {
    // Find the lyric line that corresponds to the current position
    let mut current_line = &lyrics[0];
    
    for line in lyrics {
        if line.timestamp <= position {
            current_line = line;
        } else {
            break;
        }
    }
    
    LyricLine {
        timestamp: current_line.timestamp,
        text: current_line.text.clone(),
    }
}

pub fn get_next_lyric(lyrics: &[LyricLine], position: f64) -> Option<LyricLine> {
    // Find the next lyric line after the current position
    for line in lyrics.iter() {
        if line.timestamp > position {
            return Some(LyricLine {
                timestamp: line.timestamp,
                text: line.text.clone(),
            });
        }
    }
    
    None
}

pub fn format_lyrics_for_tooltip(lyrics: &[LyricLine]) -> String {
    lyrics.iter()
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
