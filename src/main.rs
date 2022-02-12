use std::io;
use std::time::Duration;

use app::App;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use exitfailure::ExitFailure;
use handler::event::handle_event;
use rodio::OutputStream;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use view::handle_theme;

mod app;
mod commands;
mod config;
mod file_ops;
mod handler;
mod music;
mod utils;
mod view;

fn main() -> Result<(), ExitFailure> {
    let init_config = config::init()?;
    let theme = handle_theme(init_config.theme);

    // Initialize terminal
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    // Initialize App state
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let mut app = App::new(&mut terminal, &init_config.music_database, stream_handle)?;

    loop {
        app.update_window_height();
        view::draw(&mut app, &theme)?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if !handle_event(&mut app, &init_config.music_database)? {
                break;
            };
        }
        app.check_music_list();
    }

    disable_raw_mode()?;

    Ok(())
}
