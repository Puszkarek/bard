use regex::Regex;
use crate::models::lyrics::LyricLine;

pub fn parse_lyrics(lyrics_text: &str) -> Vec<LyricLine> {
    let mut lines = Vec::new();
    let timestamp_regex = Regex::new(r"\[(\d+):(\d+)\.(\d+)\](.*)").unwrap();
    
    for line in lyrics_text.lines() {
        if let Some(caps) = timestamp_regex.captures(line) {
            let minutes: f64 = caps.get(1).unwrap().as_str().parse().unwrap_or(0.0);
            let seconds: f64 = caps.get(2).unwrap().as_str().parse().unwrap_or(0.0);
            let centiseconds: f64 = caps.get(3).unwrap().as_str().parse().unwrap_or(0.0);
            let text = caps.get(4).unwrap().as_str().to_string();
            
            let timestamp = minutes * 60.0 + seconds + centiseconds / 100.0;
            
            lines.push(LyricLine {
                timestamp,
                text,
            });
        } else if !line.trim().is_empty() {
            // For non-timestamped lyrics, just add them with timestamp 0
            lines.push(LyricLine {
                timestamp: 0.0,
                text: line.to_string(),
            });
        }
    }
    
    // Sort by timestamp
    lines.sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap());
    
    lines
}
