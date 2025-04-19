#[derive(PartialEq)]
pub enum SongStatus {
    Paused,
    Playing,
}

pub struct SongInfo {
    pub id: String,
    pub artist: String,
    pub title: String,
    pub position: f64,
    pub status: SongStatus,
}
