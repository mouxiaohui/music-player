use exitfailure::ExitFailure;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitConfig {
    pub music_database: String,
    pub theme: InitTheme,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitTheme {
    pub list_title_color: String,
    pub list_title_page_color: String,
    pub list_border_color: String,
    pub list_music_color: String,
    pub list_folder_color: String,
    pub list_icon_color: String,
    pub list_selected_color: String,
    pub search_border_color: String,
    pub search_icon_color: String,
    pub search_font_color: String,
    pub command_font_color: String,
    pub command_border_color: String,
    pub music_pic_color1: String,
    pub music_pic_color2: String,
    pub usage_color_left: String,
    pub usage_color_right: String,
    pub cut_off_rule_color: String,
    pub play_music_list_title_color: String,
    pub play_music_list_border_color: String,
    pub play_music_list_id_color: String,
    pub play_music_list_duration_color: String,
    pub play_music_list_name_color: String,
    pub play_music_list_artist_color: String,
    pub play_music_list_album_color: String,
    pub play_music_list_header_color: String,
    pub playing_music_border_color: String,
    pub playing_music_name_color: String,
    pub volume_icon_color: String,
    pub volume_value_color: String,
    pub gauge_color: String,
    pub gauge_border_color: String,
    pub gauge_label_color: String,
}

fn default_init_config() -> Result<InitConfig, ExitFailure> {
    match dirs::audio_dir() {
        Some(audio_dir) => Ok(InitConfig {
            music_database: audio_dir.to_str().unwrap().to_string(),
            theme: InitTheme {
                list_title_color: "#ffaaff".to_string(),
                list_title_page_color: "#ffb747".to_string(),
                list_border_color: "#ffffff".to_string(),
                list_music_color: "#eee4c4".to_string(),
                list_folder_color: "#eee4c4".to_string(),
                list_icon_color: "#f07178".to_string(),
                list_selected_color: "#c3e88d".to_string(),
                search_border_color: "#ffb747".to_string(),
                search_icon_color: "#ec998b".to_string(),
                search_font_color: "#eee4c4".to_string(),
                command_font_color: "#eee4c4".to_string(),
                command_border_color: "#c3eead".to_string(),
                music_pic_color1: "#f07178".to_string(),
                music_pic_color2: "#81a8fd".to_string(),
                usage_color_left: "#beb2ec".to_string(),
                usage_color_right: "#eee188".to_string(),
                cut_off_rule_color: "#c3e88d".to_string(),
                play_music_list_title_color: "#81a8fd".to_string(),
                play_music_list_border_color: "#ffaaff".to_string(),
                play_music_list_id_color: "#e0d7ca".to_string(),
                play_music_list_duration_color: "#a9c34f".to_string(),
                play_music_list_name_color: "#eee4c4".to_string(),
                play_music_list_artist_color: "#b2e2e4".to_string(),
                play_music_list_album_color: "#eee188".to_string(),
                play_music_list_header_color: "#d15aa7".to_string(),
                playing_music_border_color: "#81a8fd".to_string(),
                playing_music_name_color: "#d8ce2e".to_string(),
                volume_icon_color: "#9998af".to_string(),
                volume_value_color: "#dcd8da".to_string(),
                gauge_color: "#cece68".to_string(),
                gauge_border_color: "#abcc7e".to_string(),
                gauge_label_color: "#fa4d70".to_string(),
            },
        }),
        None => panic!("The path error"),
    }
}

pub fn init() -> Result<InitConfig, ExitFailure> {
    match dirs::home_dir() {
        Some(home_path) => {
            let mut pathbuf = std::path::PathBuf::new();
            pathbuf.push(home_path.to_str().unwrap());
            pathbuf.push(".config");
            pathbuf.push("music_player");
            pathbuf.push("config.yml");

            // The configuration file does not exist, use default configuration.
            if !pathbuf.is_file() {
                return default_init_config();
            }

            let file = match std::fs::File::open(pathbuf) {
                Ok(file) => file,
                Err(_) => panic!("Configuration file not found"),
            };
            let init_config: InitConfig = serde_yaml::from_reader(file)?;

            Ok(init_config)
        }
        None => panic!("The path error"),
    }
}
