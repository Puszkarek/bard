use crate::{
    config::Config,
    models::song::{SongInfo, SongStatus},
};
use anyhow::Result;
use std::process::Command;

pub fn get_current_song(config: &Config) -> Result<Option<SongInfo>> {
    let mut args = vec![
        "metadata".to_string(),
        "--format".to_string(),
        "{{status}}\n{{artist}}\n{{title}}\n{{position}}".to_string(),
    ];

    // Add ignore-player flags for each player to ignore
    if let Some(ignore_list) = &config.ignore_players {
        if !ignore_list.is_empty() {
            let ignore_list = ignore_list.join(",");
            args.push("--ignore-player".to_string());
            args.push(ignore_list);
        }
    }

    // If allowed_players is specified, add them as a comma-separated list
    if let Some(allow_list) = &config.allowed_players {
        if !allow_list.is_empty() {
            let allow_list = allow_list.join(",");
            args.push("--player".to_string());
            args.push(allow_list);
        }
    }

    let output = Command::new("playerctl").args(args).output()?;

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
