use exitfailure::ExitFailure;
use tui::layout::{Constraint, Direction, Layout};

use crate::app::App;
use crate::config::InitTheme;

use self::color::Theme;
use self::music_list::draw_music_list;
use self::play_music_list::draw_play_music_list;
use self::playing_music::draw_playing_music;
pub mod color;
mod display;
mod music_list;
mod play_music_list;
mod playing_music;

pub fn handle_theme(init_theme: InitTheme) -> Theme {
    Theme::new(init_theme)
}

pub fn draw(app: &mut App, theme: &Theme) -> Result<(), ExitFailure> {
    let search_string = app.get_search_string();
    let command_string = app.get_command_string();
    app.terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(f.size());

        draw_music_list(
            f,
            chunks[0],
            theme,
            app.window_height as usize,
            &app.directory_contents,
            &app.selection_index,
            &search_string,
            &command_string,
            &app.error,
        );

        //Create the list chunks
        let chunks_right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(5)])
            .split(chunks[1]);

        draw_play_music_list(
            f,
            chunks_right[0],
            &theme,
            &app.play_music_list,
            &app.playing_music,
            app.player.is_paused()
        );

        draw_playing_music(
            f,
            chunks_right[1],
            &theme,
            &app.playing_music,
            app.player.is_paused(),
            app.player.volume(),
            &app.play_style
        );
    })?;

    Ok(())
}
