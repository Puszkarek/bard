use anyhow::Result;
use lofty::file::TaggedFileExt;
use lofty::read_from_path;
use regex::Regex;
use std::fs;
use std::path::Path;

use crate::lyrics::parser::parse_lyrics;
use crate::models::LyricLine;
use crate::models::SongInfo;

pub fn get_lyrics(song: &SongInfo) -> Result<Option<Vec<LyricLine>>> {
    // Try to get lyrics from ID3 tags
    if let Ok(lyrics) = get_lyrics_from_tags(&song.file_path) {
        if !lyrics.is_empty() {
            return Ok(Some(parse_lyrics(&lyrics)));
        }
    }

    // Try to find a .lrc file with the same name
    let lrc_path = Path::new(&song.file_path).with_extension("lrc");
    if lrc_path.exists() {
        let lyrics = fs::read_to_string(lrc_path)?;
        if !lyrics.is_empty() {
            return Ok(Some(parse_lyrics(&lyrics)));
        }
    }

    // No lyrics found
    Ok(None)
}

fn get_lyrics_from_tags(file_path: &str) -> Result<String> {
    let tagged_file = read_from_path(file_path)?;

    // Try different tag fields that might contain lyrics
    let tag = tagged_file.tags()[0].clone();

    for item in tag.items() {
        let item_value = item.value();

        if let Some(text) = item_value.text() {
            // Lyrics are long texts with timestamps, can't get more infor about that so we check with a regex
            let regex = Regex::new(r"\[(\d+):(\d+)\.(\d+)\](.*)").unwrap();
            if regex.is_match(text) {
                return Ok(text.to_string());
            }
        }
    }

    Ok(String::new())
}
