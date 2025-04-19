use shared::models::SongInfo;

use crate::models::WaybarOutput;

pub fn render_no_song() -> () {
    // No song playing
    let output = WaybarOutput {
        text: "No song playing".to_string(),
        alt: "".to_string(),
        tooltip: "".to_string(),
        class: "no-song".to_string(),
    };
    println!("{}", serde_json::to_string(&output).unwrap());
}

pub fn render_song_info(song_info: &SongInfo) -> () {
    let parsed_text = format!("{} - {}", song_info.artist, song_info.title);
    let output = WaybarOutput {
        text: parsed_text.to_string(),
        alt: "".to_string(),
        tooltip: parsed_text.to_string(),
        class: "has-song".to_string(),
    };
    println!("{}", serde_json::to_string(&output).unwrap());
}

pub fn render_lyrics(current_lyric_line: String, next_lyric_line: String, tooltip: String) -> () {
    let output = get_lyrics_output(current_lyric_line, next_lyric_line, tooltip);
    println!("{}", serde_json::to_string(&output).unwrap());
}

fn get_lyrics_output(
    current_lyric_line: String,
    next_lyric_line: String,
    tooltip: String,
) -> WaybarOutput {
    if current_lyric_line.is_empty() {
        return WaybarOutput {
            text: "...".to_string(),
            alt: "".to_string(),
            tooltip,
            class: "has-lyrics".to_string(),
        };
    }
    WaybarOutput {
        text: current_lyric_line,
        alt: next_lyric_line,
        tooltip,
        class: "has-lyrics".to_string(),
    }
}
