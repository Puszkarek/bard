use anyhow::Result;
use shared::lyrics::{get_lyrics, get_lyrics_status};
use shared::models::{LyricLine, SongInfo, SongStatus};
use shared::player;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use tokio::runtime::Runtime;

mod terminal;

use terminal::TerminalDisplay;

fn main() -> Result<()> {
    // Create a Tokio runtime
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    // Create terminal display
    let mut terminal = TerminalDisplay::new().expect("Failed to initialize terminal display");

    // Track current song path
    let current_song_id = Arc::new(Mutex::new(String::new()));
    let lyrics = Arc::new(Mutex::new(Result::<Option<Vec<LyricLine>>>::Ok(None)));

    // Handle keyboard input in a separate thread
    let (tx, rx) = std::sync::mpsc::channel();
    let input_handle = thread::spawn(move || {
        let stdin = io::stdin();
        for evt in stdin.keys() {
            if let Ok(key) = evt {
                if tx.send(key).is_err() {
                    break;
                }
                if key == Key::Char('q') {
                    break;
                }
            }
        }
    });

    // Main loop
    loop {
        // Check for 'q' key press to exit
        if let Ok(Key::Char('q')) = rx.try_recv() {
            break;
        }

        // Get current song info from player
        let song_info = player::get_current_song();

        match song_info {
            Ok(Some(song)) => {
                if song.id != current_song_id.lock().unwrap().as_str() {
                    // Song changed
                    current_song_id.lock().unwrap().clear();
                    current_song_id.lock().unwrap().push_str(&song.id);
                    // Update lyrics
                    *lyrics.lock().unwrap() = rt.block_on(get_lyrics(&song));
                }

                if let Err(e) = update_lyrics(&mut terminal, &lyrics.lock().unwrap(), &song) {
                    eprintln!("Error updating lyrics: {}", e);
                }
            }
            Ok(None) => {
                // No song playing
                terminal.render_no_song().expect("Failed to render no song");
                thread::sleep(Duration::from_secs(1));
                continue;
            }
            Err(e) => {
                eprintln!("Error getting current song info: {}", e);
                terminal.render_no_song().expect("Failed to render no song");
                thread::sleep(Duration::from_secs(2));
            }
        }
    }

    Ok(())
}

// Update lyrics on terminal
fn update_lyrics(
    terminal: &mut TerminalDisplay,
    lyrics_result: &Result<Option<Vec<LyricLine>>>,
    song: &SongInfo,
) -> Result<()> {
    match lyrics_result {
        Ok(Some(lyrics_data)) => {
            if song.status == SongStatus::Paused {
                terminal.render_song_info(song)?;
                thread::sleep(Duration::from_secs(1));
                return Ok(());
            }

            // Update terminal with current lyrics
            terminal.update_lyrics(&lyrics_data, song.position)?;

            // Calculate sleep duration based on next lyric timestamp
            let current_lyric = get_lyrics_status(&lyrics_data, song.position);

            if let Some(next_timestamp) = current_lyric.next_timestamp {
                let time_until_next = next_timestamp - song.position;
                if time_until_next > 0.0 {
                    // Sleep until the next lyric (with a small safety margin)
                    thread::sleep(Duration::from_secs_f64(time_until_next.max(0.01).min(1.0)));
                } else {
                    thread::sleep(Duration::from_millis(100));
                }
            } else {
                thread::sleep(Duration::from_secs(1));
            }
        }
        Ok(None) => {
            // No lyrics found
            terminal.render_song_info(song)?;
            thread::sleep(Duration::from_secs(2));
        }
        Err(e) => {
            eprintln!("Error getting lyrics: {}", e);
            terminal.render_song_info(song)?;
            thread::sleep(Duration::from_secs(2));
        }
    }

    Ok(())
}
