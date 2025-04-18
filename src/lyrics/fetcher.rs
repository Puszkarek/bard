use anyhow::Result;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use crate::lyrics::parser::parse_lyrics;
use crate::models::LyricLine;
use crate::models::SongInfo;
use crate::tidal::fetch_lyrics;

// TODO: Support config file
const LYRICS_DIR: &str = "~/lyrics";

pub async fn get_lyrics(song: &SongInfo) -> Result<Option<Vec<LyricLine>>> {
    // Expand the home directory in the path
    let lyrics_dir = LYRICS_DIR.replace("~", &std::env::var("HOME").unwrap_or_default());
    let lyrics_dir_path = Path::new(&lyrics_dir);
    // Check if the directory exists
    if !lyrics_dir_path.exists() || !lyrics_dir_path.is_dir() {
        return Ok(None);
    }

    // Try to find a .lrc file with the same name (keep original approach)
    if let Some(lyrics) = try_exact_match(lyrics_dir_path, song)? {
        return Ok(Some(lyrics));
    }

    // Try fuzzy matching if exact match fails
    if let Some(lyrics) = try_fuzzy_match(lyrics_dir_path, song)? {
        return Ok(Some(lyrics));
    }

    // Fetch lyrics from online sources
    if let Some(lyrics) = try_fetch_lyrics(lyrics_dir_path, song).await? {
        return Ok(Some(lyrics));
    }

    // No lyrics found
    Ok(None)
}

fn try_exact_match(lyrics_dir_path: &Path, song: &SongInfo) -> Result<Option<Vec<LyricLine>>> {
    let song_path = format!("{} - {}.lrc", song.artist, song.title);
    let lrc_path = lyrics_dir_path.join(&song_path);
    if lrc_path.exists() {
        let lyrics = fs::read_to_string(&lrc_path)?;
        if !lyrics.is_empty() {
            return Ok(Some(parse_lyrics(&lyrics)));
        }
    }
    Ok(None)
}

fn try_fuzzy_match(lyrics_dir_path: &Path, song: &SongInfo) -> Result<Option<Vec<LyricLine>>> {
    // Track the best match
    let mut best_match: Option<PathBuf> = None;
    let mut best_score = 0.6; // Initial minimum similarity threshold

    // Clean up artist and title for better matching
    let artist_lower = song.artist.to_lowercase();
    let title_lower = song.title.to_lowercase();

    // Walk through all .lrc files in the directory
    for entry in fs::read_dir(lyrics_dir_path)? {
        let entry = entry?;
        let path = entry.path();

        // Only process .lrc files
        if path.extension() != Some(OsStr::new("lrc")) {
            continue;
        }

        // Get filename without extension
        if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
            let filename_lower = filename.to_lowercase();

            // Basic checks for artist and title
            let contains_artist = filename_lower.contains(&artist_lower);
            let contains_title = filename_lower.contains(&title_lower);

            // Calculate a similarity score based on presence of artist and title
            let score = match (contains_artist, contains_title) {
                (true, true) => {
                    // Both artist and original title are present - likely a good match
                    let expected_filename =
                        format!("{} - {}", song.artist, song.title).to_lowercase();
                    let max_len = expected_filename.len().max(filename_lower.len()) as f32;
                    let min_len = expected_filename.len().min(filename_lower.len()) as f32;
                    0.8 + (min_len / max_len) * 0.2 // Score between 0.8 and 1.0
                }
                (true, false) => {
                    // Only artist matches - check if title contains artist (YouTube-style)
                    if title_lower.contains(&artist_lower) {
                        // Extract title without artist
                        let title_without_artist = title_lower
                            .replace(&artist_lower, "")
                            .trim()
                            .trim_start_matches('-')
                            .trim()
                            .to_string();

                        // Check if filename contains the clean title
                        if !title_without_artist.is_empty()
                            && filename_lower.contains(&title_without_artist)
                        {
                            0.75 // Good match for YouTube-style titles
                        } else {
                            0.4 // Only artist matches
                        }
                    } else {
                        0.4 // Only artist matches
                    }
                }
                (false, true) => 0.5,  // Only title matches
                (false, false) => 0.0, // Neither matches
            };

            // If it's a exact match, return immediately
            if score == 1.0 {
                let lyrics = fs::read_to_string(&path)?;
                if !lyrics.is_empty() {
                    return Ok(Some(parse_lyrics(&lyrics)));
                }
            }

            // Update best match if this is better
            if score > best_score {
                best_score = score;
                best_match = Some(path);
            }
        }
    }

    // If we found a match, read and parse the lyrics
    if let Some(path) = best_match {
        let lyrics = fs::read_to_string(path)?;
        if !lyrics.is_empty() {
            return Ok(Some(parse_lyrics(&lyrics)));
        }
    }

    Ok(None)
}

async fn try_fetch_lyrics(
    lyrics_dir_path: &Path,
    song: &SongInfo,
) -> Result<Option<Vec<LyricLine>>> {
    // Fetch lyrics from Genius API
    if let Some(lyrics) = fetch_lyrics(song).await? {
        let lyrics_path = lyrics_dir_path.join(format!("{}.lrc", song.id));
        fs::write(&lyrics_path, &lyrics)?;
        return Ok(Some(parse_lyrics(&lyrics)));
    }

    // TODO: Add more methods for fetching lyrics here

    Ok(None)
}
