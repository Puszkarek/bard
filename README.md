# cmus-waybar-lyrics

A Go-based lyrics display for [cmus](https://github.com/cmus/cmus) music player that integrates with [Waybar](https://github.com/Alexays/Waybar) to show synchronized lyrics.

## Features

- Displays current lyrics in `Waybar` based on song position
- Reads lyrics from music file tags
- Supports timestamped lyrics for precise synchronization
- Automatically scrolls lyrics based on current position in song
- Tries to guess song artist and title from file name and path
- Runs in continuous mode to keep lyrics updated
- Supports manual refresh via SIGUSR1 signal

## Requirements
- cmus music player
- Waybar
- Go (for building)

## Integration with Waybar
Add the following to your Waybar configuration file (typically `~/.config/waybar/config`):

```
 "custom/lyrics": {
    "exec": "cmus-waybar-lyrics",
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
git clone https://github.com/puszkarek/cmus-waybar-lyrics
```
Build the program:
```
cd cmus-waybar-lyrics && go build -o cmus-waybar-lyrics
```
Copy the executable to your path: 
```
sudo cp cmus-waybar-lyrics /usr/local/bin/
```

## Timestamped Lyrics Format
For best synchronization, use lyrics with timestamps in the format:

```
[MM:SS.CC]Lyrics line
```

Example:
```
[00:12.34]This is the first line
[00:15.67]This is the second line
```


## License
MIT

### Roadmap

- [ ] Add custom configuration options
- [ ] Fetch lyrics from online sources
- [ ] Add support for more music players