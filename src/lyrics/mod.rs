pub mod fetcher;
pub mod parser;
pub mod display;

pub use fetcher::get_lyrics;
pub use display::{get_current_lyric, get_next_lyric,format_lyrics_for_tooltip};
