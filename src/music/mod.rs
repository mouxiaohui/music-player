use std::time::{Instant, Duration};

use crate::file_ops::read_audio_file;
use crate::utils::split_path::split_path_to_name;

#[derive(PartialEq, Clone)]
pub struct Music {
    pub path: String,
    pub name: String,
    pub artist: String,
    pub title: String,
    pub album: String,
    pub play_position: Duration,
    pub total_duration: Duration,
    pub start_time: Option<Instant>,
}

impl Music {
    pub fn new(path: &str) -> Result<Music, String> {
        let mut file_split = split_path_to_name(&path).split('.');
        let name = file_split.next().unwrap().to_string();
        let extension = file_split.next().unwrap();

        match read_audio_file(path, extension) {
            Ok(audio) => Ok(Music {
                path: path.to_string(),
                name,
                artist: audio.artist,
                title: audio.title,
                album: audio.album,
                play_position: Duration::from_secs(0),
                total_duration: audio.duration,
                start_time: None,
            }),
            Err(err) => Err(err),
        }
    }
}
