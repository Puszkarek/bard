# B.A.R.D - Ballad Assistant Rhythm Debugger

A Rust based lyrics display for music player to show synchronized lyrics.

## Features

- Displays lyrics in a terminal window
- Displays current lyrics in [Waybar](https://github.com/Alexays/Waybar) based on song position
- Reads lyrics from music file tags
- Fetches lyrics from online sources if not found in file tags
- Saves lyrics to file for future use
- Supports multiple music players
- Supports timestamped lyrics for precise synchronization
- Automatically scrolls lyrics based on current position in song
- Tries to guess song artist and title from file name and path
- Runs in continuous mode to keep lyrics updated

## Requirements
- playerctl
- music player daemon
- Rust (for building)

## Optionals
- Waybar

## Run on terminal
Just run `bard`

## Run with Waybar
Add the following to your Waybar configuration file (typically `~/.config/waybar/config`):

```
 "custom/lyrics": {
    "exec": "waybar-bard",
    "format": "{} <span font='11' fgalpha='50%' style='italic'>{alt}</span>",
    "restart-interval": 5,
    "return-type": "json",
    "signal": 1,  // SIGRTMIN+1
    "tooltip": true
}
```

Then add styling in your Waybar CSS file:

```
#custom-lyrics {
    padding: 0 10px;
    color: #ffffff;
}

#custom-lyrics.no-song {
    color: #888888;
}

#custom-lyrics.no-lyrics {
    color: #aaaaaa;
    font-style: italic;
}

#custom-lyrics.has-lyrics {
    color: #ffffff;
}
```

## Installing
Clone this repository:
```
git clone https://github.com/puszkarek/bard
```
Install the program:
```
cd bard && sudo make install
```

## Timestamped Lyrics Format
For best synchronization, use lyrics with timestamps in the format:

```
[MM:SS.CC] Lyrics line
```

Example:
```
[00:12.34] This is the first line
[00:15.67] This is the second line
```

## Configuration

The application uses a JSON configuration file located at `~/.config/bard/config.json`. The file is automatically created with default values on first run.

### Configuration File Location

The config file is stored at:
```
~/.config/bard/config.json
```

### Configuration Options

The configuration file contains the following options:

| Option | Description | Default |
|--------|-------------|---------|
| `tidal_token` | Your Tidal API token for fetching song information | Empty string (must be set by user) |
| `lyrics_folder` | Directory where lyrics files are stored | `~/lyrics` |
| `colors` | Terminal UI color configuration | See below |

#### Color Configuration

The `colors` section contains the following options:

| Option | Description | Default |
|--------|-------------|---------|
| `default_fg` | Default text color for non-focused elements | `"gray"` |
| `focused_fg` | Text color for currently active/focused elements | `"white"` |

### Example Configuration

```json
{
  "tidal_token": "your-tidal-token-here",
  "lyrics_folder": "/home/user/lyrics",
  "colors": {
    "default_fg": "gray",
    "focused_fg": "white"
  }
}
```

## Supported Online Sources

#### Tidal
To get your Tidal API key:
1. Log in into your browser and open the developer console
2. Go to the Network tab
3. Search for "tidal"
4. Find any request with "tidal.com" that contains a Bearer token
5. Copy the token and paste it to the config file inside `~/.config/bard/config.json`

## License
MIT

### Roadmap

- [ ] Add loading indicator when fetching lyrics