use std::fs::read_dir;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

use audiotags::{AudioTag, Tag};
use rodio::decoder::DecoderError;
use rodio::Decoder;
use rodio::Source;

use crate::app::App;
use crate::utils::split_path::split_path_to_name;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone)]
pub enum DirectoryItem {
    // File(path)
    File(String),
    // Directory(path, is_open, content_len)
    Directory(String),
}

pub struct Audio {
    pub artist: String,
    pub title: String,
    pub album: String,
    pub duration: Duration,
}

impl Audio {
    fn new(tag: Box<dyn AudioTag>, duration: Duration) -> Audio {
        let artist = match tag.artist() {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };
        let title = match tag.title() {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };
        let album = match tag.album() {
            Some(album) => album.title.to_string(),
            None => "".to_string(),
        };

        Audio {
            artist,
            title,
            album,
            duration,
        }
    }
}

pub fn get_files_for_current_directory(app: &mut App) -> Result<Vec<DirectoryItem>, io::Error> {
    //Get list, unwrap, and convert results to &Path
    let dir_items: Vec<PathBuf> = match read_dir(&app.current_directory) {
        Ok(val) => val.map(|f| f.unwrap().path()).collect(),
        Err(err) => return Err(err),
    };

    //Convert items to DirectoryItem
    let mut files: Vec<DirectoryItem> = Vec::new();
    for item in dir_items {
        if item.is_file() {
            // Check whether it is an audio file
            if check_audio_file(&item)? {
                let file = DirectoryItem::File(String::from(item.to_string_lossy()));
                files.push(file);
            }
        } else {
            let file = DirectoryItem::Directory(String::from(item.to_string_lossy()));
            files.push(file);
        }
    }

    Ok(files)
}

pub fn get_files_for_current_directory_astrict(
    app: &mut App,
    astrict: &str,
) -> Result<Vec<DirectoryItem>, io::Error> {
    let astrict_low = astrict.to_ascii_lowercase();
    //Get list, unwrap, and convert results to &Path
    let dir_items: Vec<PathBuf> = match read_dir(&app.current_directory) {
        Ok(val) => val.map(|f| f.unwrap().path()).collect(),
        Err(err) => return Err(err),
    };

    //Convert items to DirectoryItem
    let mut files: Vec<DirectoryItem> = Vec::new();
    for item in dir_items {
        let path_string = String::from(item.to_string_lossy());
        if !split_path_to_name(&path_string)
            .to_ascii_lowercase()
            .contains(&astrict_low)
        {
            continue;
        };
        if item.is_file() {
            // Check whether it is an audio file
            if check_audio_file(&item)? {
                let file = DirectoryItem::File(path_string);
                files.push(file);
            }
        } else {
            let file = DirectoryItem::Directory(path_string);
            files.push(file);
        }
    }

    Ok(files)
}

pub fn check_audio_file(path: &PathBuf) -> Result<bool, io::Error> {
    if let Some(t) = infer::get_from_path(path.to_str().unwrap())? {
        let mime_type = t.mime_type();

        return Ok(mime_type.contains("audio") || mime_type.contains("video"));
    }

    Ok(false)
}

pub fn read_audio_file<'a>(path: &str, extension: &str) -> Result<Audio, String> {
    let tag = match Tag::default().read_from_path(path) {
        Ok(tag) => tag,
        Err(err) => return Err(err.to_string()),
    };

    let mut duration = Duration::from_secs(0);
    if extension == "MP3" || extension == "mp3" {
        match mp3_duration::from_path(path) {
            Ok(d) => duration = d,
            Err(err) => return Err(err.to_string()),
        };
    } else {
        match get_audio_source(path) {
            Ok(source) => {
                if let Some(d) = source.total_duration() {
                    duration = d;
                };
            }
            Err(err) => return Err(err.to_string()),
        };
    };

    Ok(Audio::new(tag, duration))
}

pub fn get_audio_source(path: &str) -> Result<Decoder<File>, DecoderError> {
    let file = File::open(path).unwrap();
    Decoder::new(file)
}
