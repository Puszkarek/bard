use crate::models::song::{SongInfo, SongStatus};
use anyhow::Result;
use std::process::Command;

pub fn get_current_song() -> Result<Option<SongInfo>> {
    let output = Command::new("playerctl")
        .args([
            "metadata",
            "--format",
            "{{status}}\n{{artist}}\n{{title}}\n{{position}}",
        ])
        .output()?;

    if !output.status.success() {
        return Ok(None);
    }

    let output_str = String::from_utf8(output.stdout)?;

    // Extract song information
    let lines: Vec<&str> = output_str.lines().collect();
    if lines.len() < 4 {
        return Ok(None);
    }

    let status = match lines[0] {
        "Playing" => SongStatus::Playing,
        _ => SongStatus::Paused,
    };
    let artist = lines[1].to_string();
    let title = lines[2].to_string();
    let duration = std::time::Duration::from_millis(lines[3].parse::<u64>()? / 1000);
    let position = duration.as_secs_f64();
    let id = format!("{} - {}", artist, title);

    Ok(Some(SongInfo {
        id,
        artist,
        title,
        position,
        status,
    }))
}
