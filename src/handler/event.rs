use crossterm::event::{self, Event, KeyCode};
use exitfailure::ExitFailure;

use crate::app::{self, App, Mode};

pub fn handle_event(app: &mut App, music_database: &str) -> Result<bool, ExitFailure> {
    let mut is_loop = true;

    if app.error != None {
        app.error = None;
    }

    //Handle input
    if let Event::Key(key) = event::read().unwrap() {
        if app.mode == Mode::Browse {
            match key.code {
                KeyCode::Char('q') => is_loop = false,
                KeyCode::Char('g') => app.move_select_top(),
                KeyCode::Char('G') => app.move_select_bottom(),
                KeyCode::Down => app.move_select_down(1),
                KeyCode::Char('j') => app.move_select_down(1),
                KeyCode::Char('J') => app.move_select_down(5),
                KeyCode::Up => app.move_select_up(1),
                KeyCode::Char('k') => app.move_select_up(1),
                KeyCode::Char('K') => app.move_select_up(5),
                KeyCode::Char('n') => app.next_page(),
                KeyCode::Char('N') => app.previous_page(),
                KeyCode::Char('l') => app.open_folder(),
                KeyCode::Char('h') => app.back_previous_folder(music_database),
                KeyCode::Enter => app.add_music_to_list(),
                KeyCode::Char(' ') => app.stop_or_start_play(),
                KeyCode::Char('-') => app.update_volume(&|v| if v > 0.0 {v - 0.05} else {0.0}),
                KeyCode::Char('=') => app.update_volume(&|v| if v < 1.25 {v + 0.05} else {1.25}),
                KeyCode::Char('+') => app.update_volume(&|v| if v < 1.25 {v + 0.05} else {1.25}),
                KeyCode::Right => app.skip_duration(10),
                KeyCode::Left => app.skip_duration(-10),
                KeyCode::Char('|') => app.set_mode(Mode::Search),
                KeyCode::Char(':') => app.set_mode(Mode::Command),
                KeyCode::Esc => {
                    app.populate_files()?;
                    app.search_buffer = Vec::new();
                }
                _ => {}
            }
        }

        if app.mode == Mode::Search {
            match key.code {
                KeyCode::Char(chr) => app.add_to_search_buffer(chr),
                KeyCode::Enter => app.execute_search(),
                KeyCode::Backspace => {
                    if app.search_buffer.len() > 1 {
                        app.search_buffer.truncate(app.search_buffer.len() - 1);
                    };
                }
                KeyCode::Esc => {
                    app.set_mode(app::Mode::Browse);
                    app.search_buffer = Vec::new();
                }
                _ => {}
            }
        }

        if app.mode == app::Mode::Command {
            match key.code {
                KeyCode::Char(chr) => app.add_to_command_buffer(chr),
                KeyCode::Enter => app.execute_command(),
                KeyCode::Backspace => {
                    if app.command_buffer.len() > 1 {
                        app.command_buffer.truncate(app.command_buffer.len() - 1);
                    };
                }
                KeyCode::Esc => {
                    app.set_mode(app::Mode::Browse);
                    app.command_buffer = Vec::new();
                }
                _ => {}
            }
        }
    }

    Ok(is_loop)
}
