#[derive(PartialEq)]
pub enum SongStatus {
    Paused,
    Playing,
}

pub struct SongInfo {
    pub artist: String,
    pub title: String,
    pub file_path: String,
    pub position: f64,
    pub status: SongStatus,
}
