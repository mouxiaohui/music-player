use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

use crate::music::Music;

use super::color::Theme;

#[rustfmt::skip]
const MUSIC_PLAYER_PIC: &[&str] = &[
    "███╗   ███╗","██╗   ██╗","███████╗","██╗"," ██████╗", "",
    "████╗ ████║","██║   ██║","██╔════╝","██║","██╔════╝", "",
    "██╔████╔██║","██║   ██║","███████╗","██║","██║     ", "",
    "██║╚██╔╝██║","██║   ██║","╚════██║","██║","██║     ", "",
    "██║ ╚═╝ ██║","╚██████╔╝","███████║","██║","╚██████╗", "",
    "╚═╝     ╚═╝"," ╚═════╝ ","╚══════╝","╚═╝"," ╚═════╝",
];
const CUT_OFF_RULE: &str = "   ";
#[rustfmt::skip]
const USAGE: &[&str] = &[
    "Move selection up              ", "[k, K] ", " ",
    "Move selection down            ", "[j, J] ", " ",
    "Move selection to the top      ", "[g]    ", " ",
    "Move selection to the bottom   ", "[G]    ", " ",
    "Next page                      ", "[n]    ", " ",
    "Previous page                  ", "[N]    ", " ",
    "Open folder                    ", "[l]    ", " ",
    "Back to previous folder        ", "[h]    ", " ",
    "Enter search mode              ", "[|]    ", " ",
    "Enter command mode             ", "[:]    ", " ",
    "Exit program                   ", "[q]    ", " ",
    "Exit search or command mode    ", "[Esc]  ", " ",
    "Pause or resume the music      ", "[Space]", " ",
    "Decrease volume                ", "[-]    ", " ",
    "Increase volume                ", "[+, =] ", " ",
    "Add music to the playlist      ", "[Enter]",
];

pub fn draw_play_music_list<B: Backend>(
    frame: &mut Frame<B>,
    area: Rect,
    theme: &Theme,
    music_list: &Vec<Music>,
    playing_music: &Option<Music>,
    is_paused: bool,
) {
    let mut all_music_dur: u64 = 0;
    for music in music_list {
        all_music_dur += music.total_duration.as_secs();
    }
    if let Some(playing_music) = playing_music {
        let playing_total_dur = playing_music.total_duration.as_secs();
        let playing_position_dur = playing_music.play_position.as_secs();
        if playing_position_dur <= playing_total_dur {
            all_music_dur += playing_total_dur - playing_position_dur;
        }
    }
    let all_music_dur_str = format!(
        "{}h {:0>2}m {:>2}s ",
        (all_music_dur / 60 / 60),
        (all_music_dur / 60 % 60),
        (all_music_dur % 60),
    );
    let mut title_spans = Vec::new();
    // let mut title_spans = Spans::default();
    if music_list.len() > 0 || playing_music != &None {
        let mut total_music = music_list.len();
        if playing_music != &None {
            total_music += 1;
        }

        title_spans.push(Span::styled(
            " Play list ",
            Style::default().fg(theme.play_music_list_title_color),
        ));
        title_spans.push(Span::styled(" | ", Style::default().fg(Color::Yellow)));
        title_spans.push(Span::styled(
            format!(" {} songs ", total_music),
            Style::default().fg(theme.play_music_list_title_color),
        ));
        title_spans.push(Span::styled(" | ", Style::default().fg(Color::Yellow)));
        title_spans.push(Span::styled(
            format!(" {} ", all_music_dur_str),
            Style::default().fg(theme.play_music_list_title_color),
        ));
    };

    // Play music list block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(title_spans)
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(theme.play_music_list_border_color));
    frame.render_widget(block, area);

    if music_list.len() > 0 || playing_music != &None {
        draw_play_list(frame, &area, theme, music_list, &playing_music, is_paused);
    } else {
        draw_home_page(frame, &area, theme);
    }
}

fn draw_home_page<B: Backend>(frame: &mut Frame<B>, area: &Rect, theme: &Theme) {
    let inner_rect = Rect::new(area.x + 1, area.y + 2, area.width - 2, area.height - 3);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Min(3)])
        .split(inner_rect);

    // Display music picture
    let mut pic: Vec<Spans> = Vec::new();
    let pic_split = MUSIC_PLAYER_PIC.split(|s| s.len() == 0);
    for line in pic_split {
        let mut spans: Vec<Span> = Vec::new();
        for (i, item) in line.iter().enumerate() {
            let mut color = theme.music_pic_color1;
            if i % 2 == 0 {
                color = theme.music_pic_color2;
            }
            spans.push(Span::styled(*item, Style::default().fg(color)))
        }
        pic.push(Spans::from(spans));
    }
    frame.render_widget(Paragraph::new(pic).alignment(Alignment::Center), chunks[0]);

    // Display usage
    let mut usage: Vec<Spans> = vec![Spans::from(Span::styled(
        CUT_OFF_RULE,
        Style::default().fg(theme.cut_off_rule_color),
    ))];
    let usage_split = USAGE.split(|s| *s == " ");
    for line in usage_split {
        usage.push(Spans::from(vec![
            Span::styled(line[0], Style::default().fg(theme.usage_color_left)),
            Span::styled(line[1], Style::default().fg(theme.usage_color_right)),
        ]))
    }

    frame.render_widget(
        Paragraph::new(usage).alignment(Alignment::Center),
        chunks[1],
    );
}

fn draw_play_list<B: Backend>(
    frame: &mut Frame<B>,
    area: &Rect,
    theme: &Theme,
    music_list: &Vec<Music>,
    playing_music: &Option<Music>,
    is_paused: bool,
) {
    let mut names: Vec<Spans> = Vec::new();
    let mut artists: Vec<Spans> = Vec::new();
    let mut albums: Vec<Spans> = Vec::new();

    if let Some(playing_music) = playing_music {
        let playing_music_s = playing_music.total_duration.as_secs() % 60;
        let playing_music_m = playing_music.total_duration.as_secs() / 60;
        let mut lable = " ";
        if is_paused {
            lable = " "
        }
        names.push(Spans::from(vec![
            Span::styled(
                format!("{:>3}", lable),
                Style::default().fg(theme.gauge_label_color),
            ),
            Span::styled(
                format!("[{:0>2}m {:2}s]", playing_music_m, playing_music_s),
                Style::default().fg(theme.play_music_list_duration_color),
            ),
            Span::styled(
                &playing_music.name,
                Style::default()
                    .fg(theme.playing_music_name_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        artists.push(Spans::from(vec![Span::styled(
            format!(" {}", &playing_music.artist),
            Style::default().fg(theme.play_music_list_artist_color),
        )]));

        albums.push(Spans::from(vec![Span::styled(
            format!(" {}", &playing_music.album),
            Style::default().fg(theme.play_music_list_album_color),
        )]));
    }

    for (i, music) in music_list.iter().enumerate() {
        let s = music.total_duration.as_secs() % 60;
        let m = music.total_duration.as_secs() / 60;
        names.push(Spans::from(vec![
            Span::styled(
                format!("{:>2}.", i + 1),
                Style::default().fg(theme.play_music_list_id_color),
            ),
            Span::styled(
                format!("[{:0>2}m {:2}s]", m, s),
                Style::default().fg(theme.play_music_list_duration_color),
            ),
            Span::styled(
                &music.name,
                Style::default().fg(theme.play_music_list_name_color),
            ),
        ]));

        artists.push(Spans::from(vec![Span::styled(
            format!(" {}", &music.artist),
            Style::default().fg(theme.play_music_list_artist_color),
        )]));

        albums.push(Spans::from(vec![Span::styled(
            format!(" {}", &music.album),
            Style::default().fg(theme.play_music_list_album_color),
        )]));
    }

    names.insert(
        0,
        Spans::from(vec![Span::styled(
            format!("{: >12}Name", ""),
            Style::default().fg(theme.play_music_list_header_color),
        )]),
    );
    artists.insert(
        0,
        Spans::from(vec![Span::styled(
            " Artist",
            Style::default().fg(theme.play_music_list_header_color),
        )]),
    );
    albums.insert(
        0,
        Spans::from(vec![Span::styled(
            " Album",
            Style::default().fg(theme.play_music_list_header_color),
        )]),
    );

    //Create the list chunks
    let inner_rect = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Percentage(20),
            Constraint::Percentage(35),
        ])
        .split(inner_rect);

    frame.render_widget(Paragraph::new(names), chunks[0]);
    frame.render_widget(Paragraph::new(artists), chunks[1]);
    frame.render_widget(Paragraph::new(albums), chunks[2]);
}
