use anyhow::{Result, anyhow};
use regex::Regex;
use std::path::Path;
use std::process::Command;
use crate::models::song::SongInfo;

pub fn get_current_song() -> Result<Option<SongInfo>> {
    let output = Command::new("cmus-remote")
        .arg("-Q")
        .output()?;
    
    if !output.status.success() {
        return Err(anyhow!("Failed to execute cmus-remote"));
    }
    
    let output_str = String::from_utf8(output.stdout)?;
    
    // Check if cmus is playing or paused
    if !output_str.contains("status playing") && !output_str.contains("status paused") {
        return Ok(None);
    }
    
    // Extract song information
    let mut artist = String::new();
    let mut title = String::new();
    let mut file_path = String::new();
    let mut position = 0.0;
    let mut duration = 0.0;
    
    for line in output_str.lines() {
        if line.starts_with("tag artist ") {
            artist = line[11..].to_string();
        } else if line.starts_with("tag title ") {
            title = line[10..].to_string();
        } else if line.starts_with("file ") {
            file_path = line[5..].to_string();
        } else if line.starts_with("position ") {
            position = line[9..].parse::<f64>()?;
        } else if line.starts_with("duration ") {
            duration = line[9..].parse::<f64>()?;
        }
    }
    
    // If artist or title is missing, try to extract from filename
    if artist.is_empty() || title.is_empty() {
        let file_name = Path::new(&file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        // Try to match "Artist - Title" pattern
        let re = Regex::new(r"(?i)([^-]+)\s*-\s*(.+)\.[^.]+$").unwrap();
        if let Some(caps) = re.captures(file_name) {
            if artist.is_empty() && caps.get(1).is_some() {
                artist = caps.get(1).unwrap().as_str().trim().to_string();
            }
            if title.is_empty() && caps.get(2).is_some() {
                title = caps.get(2).unwrap().as_str().trim().to_string();
            }
        }
    }
    
    Ok(Some(SongInfo {
        artist,
        title,
        file_path,
        position,
        duration,
    }))
}
