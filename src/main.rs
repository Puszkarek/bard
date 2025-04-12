use anyhow::Result;
use signal_hook::consts::SIGUSR1;
use signal_hook::iterator::Signals;
use std::thread;
use std::time::Duration;
use models::WaybarOutput;

mod lyrics;
mod cmus;
mod models;



fn main() -> Result<()> {
    // Set up signal handling for SIGUSR1
    let mut signals = Signals::new(&[SIGUSR1])?;
    
    let handle = thread::spawn(move || {
        for _ in signals.forever() {
            // Refresh lyrics when receiving SIGUSR1
            if let Err(e) = update_lyrics() {
                eprintln!("Error refreshing lyrics: {}", e);
            }
        }
    });

    // Main loop
    loop {
        if let Err(e) = update_lyrics() {
            eprintln!("Error updating lyrics: {}", e);
        }
        thread::sleep(Duration::from_secs(1));
    }

}

fn update_lyrics() -> Result<()> {
    // Get current song info from cmus
    let song_info = cmus::get_current_song()?;
    
    if song_info.is_none() {
        // No song playing
        let output = WaybarOutput {
            text: "No song playing".to_string(),
            alt: "".to_string(),
            tooltip: "".to_string(),
            class: "no-song".to_string(),
        };
        println!("{}", serde_json::to_string(&output)?);
        return Ok(());
    }
    
    let song = song_info.unwrap();
    
    // Get lyrics for the current song
    let lyrics_result = lyrics::get_lyrics(&song);
    
    match lyrics_result {
        Ok(Some(lyrics_data)) => {
            // Find current lyric based on position
            let current_lyric = lyrics::get_current_lyric(&lyrics_data, song.position);
            let next_lyric = lyrics::get_next_lyric(&lyrics_data, song.position);
            
            let output = WaybarOutput {
                text: current_lyric.text,
                alt: next_lyric.map_or("".to_string(), |l| l.text),
                tooltip: lyrics::format_lyrics_for_tooltip(&lyrics_data),
                class: "has-lyrics".to_string(),
            };
            println!("{}", serde_json::to_string(&output)?);
        },
        Ok(None) => {
            // No lyrics found
            let output = WaybarOutput {
                text: format!("{} - {}", song.artist, song.title),
                alt: "No lyrics found".to_string(),
                tooltip: "No lyrics available for this song".to_string(),
                class: "no-lyrics".to_string(),
            };
            println!("{}", serde_json::to_string(&output)?);
        },
        Err(e) => {
            // Error getting lyrics
            let output = WaybarOutput {
                text: format!("{} - {}", song.artist, song.title),
                alt: "Error getting lyrics".to_string(),
                tooltip: format!("Error: {}", e),
                class: "no-lyrics".to_string(),
            };
            println!("{}", serde_json::to_string(&output)?);
        }
    }
    
    Ok(())
}
