# cmd-lyrics-rs

A Rust based lyrics display for music player that integrates with [Waybar](https://github.com/Alexays/Waybar) to show synchronized lyrics.

## Features

- Displays lyrics in a terminal window
- Displays current lyrics in `Waybar` based on song position
- Reads lyrics from music file tags
- Fetches lyrics from online sources if not found in file tags
- Saves lyrics to file for future use
- Supports multiple music players
- Supports timestamped lyrics for precise synchronization
- Automatically scrolls lyrics based on current position in song
- Tries to guess song artist and title from file name and path
- Runs in continuous mode to keep lyrics updated

## Requirements
- music player daemon (e.g. cmus)
- Rust (for building)
# Optionals
- Waybar

## Run on terminal
TODO

## Run with Waybar
Add the following to your Waybar configuration file (typically `~/.config/waybar/config`):

```
 "custom/lyrics": {
    "exec": "waybar-lyrics-rs",
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
git clone https://github.com/puszkarek/cmd-lyrics-rs
```
Install the program:
```
cd cmd-lyrics-rs && sudo make install
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

## Supported Online Sources

#### Tidal
To get your Tidal API key:
1. Log in into your browser and open the developer console
2. Go to the Network tab
3. Search for "tidal"
4. Find any request with "tidal.com" that contains a Bearer token
5. Copy the token and paste it to `.env` file

## License
MIT

### Roadmap

- [ ] Add custom configuration options