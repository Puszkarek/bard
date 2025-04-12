use anyhow::Result;
use models::{LyricLine, SongInfo};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod cmus;
mod lyrics;
mod models;

fn main() -> Result<()> {
    // Track current song path
    let current_song_path = Arc::new(Mutex::new(String::new()));
    let lyrics = Arc::new(Mutex::new(Result::<Option<Vec<LyricLine>>>::Ok(None)));

    // Main loop
    loop {
        // Get current song info from cmus
        let song_info = cmus::get_current_song()?;
        if song_info.is_none() {
            cmus::render_no_song();
            continue;
        }

        let unwrapped_song_info = song_info.unwrap();
        if unwrapped_song_info.file_path != current_song_path.lock().unwrap().as_str() {
            // Song changed
            current_song_path.lock().unwrap().clear();
            current_song_path
                .lock()
                .unwrap()
                .push_str(&unwrapped_song_info.file_path);
            // Update lyrics
            let new_lyrics = lyrics::get_lyrics(&unwrapped_song_info);
            *lyrics.lock().unwrap() = new_lyrics;
        }

        if let Err(e) = update_lyrics(&lyrics.lock().unwrap(), &unwrapped_song_info) {
            eprintln!("Error updating lyrics: {}", e);
        }
    }
}

fn update_lyrics(lyrics_result: &Result<Option<Vec<LyricLine>>>, song: &SongInfo) -> Result<()> {
    // Get lyrics for the current song

    match lyrics_result {
        Ok(Some(lyrics_data)) => {
            // Find current lyric based on position
            let current_lyric = lyrics::get_lyrics_status(&lyrics_data, song.position);
            let tooltip = lyrics::format_lyrics_for_tooltip(&lyrics_data);

            cmus::render_lyrics(current_lyric.current_line, current_lyric.next_line, tooltip);
            thread::sleep(Duration::from_secs(1));
        }
        Ok(None) => {
            // No lyrics found
            cmus::render_song_info(song);
            thread::sleep(Duration::from_secs(2));
        }
        Err(e) => {
            eprintln!("Error getting lyrics: {}", e);
            // Error getting lyrics
            cmus::render_song_info(song);
            thread::sleep(Duration::from_secs(2));
        }
    }

    Ok(())
}
