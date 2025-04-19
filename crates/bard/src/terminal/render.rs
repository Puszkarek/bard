use ratatui::style::Color;
use ratatui::{
    Terminal,
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::Paragraph,
};
use shared::config::Config;
use shared::models::{LyricLine, SongInfo};
use std::io::{self};
use termion::screen::IntoAlternateScreen;
use termion::{
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};

pub struct TerminalDisplay {
    pub terminal: Terminal<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>>,
    pub default_color: Color,
    pub focused_color: Color,
}

// TODO: Move these methods to a separate folder inside this module src
const POSITION_OFFSET_SECONDS: f64 = 1.0;
fn get_current_line_index(lyrics: &[LyricLine], position: f64) -> usize {
    // Include offset in position comparison
    let adjusted_position = position + POSITION_OFFSET_SECONDS;

    let current_index = lyrics
        .iter()
        .enumerate()
        .take_while(|(_, line)| line.timestamp <= adjusted_position)
        .map(|(i, _)| i)
        .last();

    return current_index.unwrap_or(0);
}

pub fn parse_color(color_str: &str) -> Color {
    match color_str.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "gray" => Color::Gray,
        "darkgray" => Color::DarkGray,
        "lightred" => Color::LightRed,
        "lightgreen" => Color::LightGreen,
        "lightyellow" => Color::LightYellow,
        "lightblue" => Color::LightBlue,
        "lightmagenta" => Color::LightMagenta,
        "lightcyan" => Color::LightCyan,
        "white" => Color::White,
        _ => Color::White, // Default to white for invalid colors
    }
}

impl TerminalDisplay {
    pub fn new() -> Result<Self, io::Error> {
        // First create a raw terminal
        let raw_terminal = io::stdout().into_raw_mode()?;
        // Then wrap it in AlternateScreen
        let stdout = AlternateScreen::from(raw_terminal.into_alternate_screen()?);
        // Now create the backend with the AlternateScreen
        let backend = TermionBackend::new(stdout);
        // Finally create the terminal with the backend
        let terminal = Terminal::new(backend)?;

        let config = Config::load();
        match config {
            Ok(config) => {
                let default_color = parse_color(&config.colors.default_fg);
                let focused_color = parse_color(&config.colors.focused_fg);
                // Return the TerminalDisplay instance
                Ok(Self {
                    terminal,
                    default_color,
                    focused_color,
                })
            }
            Err(e) => {
                eprintln!("Error loading config: {}", e);
                Err(io::Error::new(io::ErrorKind::Other, "Error loading config"))
            }
        }
    }

    pub fn update_lyrics(
        &mut self,
        lyrics: &Vec<LyricLine>,
        current_position: f64,
    ) -> io::Result<()> {
        let current_index = get_current_line_index(lyrics, current_position);
        self.render_lyrics(lyrics, current_index)
    }

    pub fn render_lyrics(
        &mut self,
        lyrics: &Vec<LyricLine>,
        current_index: usize,
    ) -> io::Result<()> {
        self.terminal.draw(|frame| {
            let size = frame.area();

            // Calculate how many lines can fit in the terminal
            let available_height = size.height as usize;

            // Reserve some space for potential borders/margins
            let max_displayable_lines = available_height.saturating_sub(2);

            // Calculate how many lines to show before/after the current line
            let lines_before = max_displayable_lines / 2;
            let lines_after = max_displayable_lines - lines_before;

            // Calculate visible lyrics range
            let start_index = current_index.saturating_sub(lines_before);
            let end_index = (current_index + lines_after).min(lyrics.len());
            let visible_lyrics = &lyrics[start_index..end_index];

            // Create a list of styled spans for each visible lyric line
            let mut text_lines = Vec::new();

            for (i, lyric) in visible_lyrics.iter().enumerate() {
                let actual_index = i + start_index;
                let style = if actual_index == current_index {
                    Style::default()
                        .fg(self.focused_color)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(self.default_color)
                };

                text_lines.push(Line::from(Span::styled(&lyric.text, style)));
            }

            let text_lines_len = text_lines.len() as u16;
            let text = Text::from(text_lines);
            let paragraph = Paragraph::new(text).alignment(Alignment::Center);

            // Calculate empty space to center content vertically
            let empty_space = size.height.saturating_sub(text_lines_len);
            let top_padding = empty_space / 2;
            let bottom_padding = empty_space - top_padding;

            // Create a vertical layout with calculated padding for centering
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(top_padding),    // Top padding
                        Constraint::Length(text_lines_len), // Exact content height
                        Constraint::Length(bottom_padding), // Bottom padding
                    ]
                    .as_ref(),
                )
                .split(size);

            frame.render_widget(paragraph, chunks[1]);
        })?;

        Ok(())
    }

    pub fn render_song_info(&mut self, song: &SongInfo) -> io::Result<()> {
        self.terminal.draw(|frame| {
            let size = frame.area();

            // Apply the current_line color from config to make it stand out
            let styled_text = Text::from(Span::styled(
                format!("{} - {}", song.artist, song.title),
                Style::default()
                    .fg(self.default_color)
                    .add_modifier(Modifier::BOLD),
            ));

            let paragraph = Paragraph::new(styled_text).alignment(Alignment::Center);

            // Center vertically
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(45),
                        Constraint::Min(3),
                        Constraint::Percentage(45),
                    ]
                    .as_ref(),
                )
                .split(size);

            frame.render_widget(paragraph, chunks[1]);
        })?;

        Ok(())
    }

    pub fn render_no_song(&mut self) -> io::Result<()> {
        self.terminal.draw(|frame| {
            let size = frame.area();

            // Apply the other_lines color from config
            let styled_text = Text::from(Span::styled(
                "No song playing",
                Style::default().fg(self.default_color),
            ));

            let paragraph = Paragraph::new(styled_text).alignment(Alignment::Center);

            // Center vertically
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(45),
                        Constraint::Min(3),
                        Constraint::Percentage(45),
                    ]
                    .as_ref(),
                )
                .split(size);

            frame.render_widget(paragraph, chunks[1]);
        })?;

        Ok(())
    }
}
