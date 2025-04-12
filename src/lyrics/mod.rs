pub mod display;
pub mod fetcher;
pub mod parser;

pub use display::{format_lyrics_for_tooltip, get_lyrics_status};
pub use fetcher::get_lyrics;
