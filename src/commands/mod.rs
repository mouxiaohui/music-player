use crate::app::{App, PlayStyle};

pub fn process_command(app: &mut App, command_string: String) {
    //split command buffer
    let mut splist_command: Vec<String> = command_string
        .trim_start_matches(":")
        .split_ascii_whitespace()
        .map(String::from)
        .collect();

    if splist_command.len() == 0 {
        splist_command.append(&mut vec![String::new()]);
    }
    match splist_command[0].to_ascii_uppercase().as_ref() {
        "REMOVE" | "RM" => remove_command(app, splist_command),
        "CLEAR" | "CLS" => app.clear_play_music_list(),
        "ALL" => app.add_all_music_to_list(),
        "ORDER" | "OD" => app.set_play_style(PlayStyle::PlayOrder),
        "SINGLECYCLE" | "SC" => app.set_play_style(PlayStyle::SingleCycle),
        "NEXT" | "N" => app.play_next_music(),
        "SHUFFLE" | "SH" => app.shuffle_playlist(),
        _ => app.error = Some(String::from("Not a command")),
    }

    app.move_select_up(1);
}

fn remove_command(app: &mut App, splist_command: Vec<String>) {
    let mut remove_index: Vec<usize> = Vec::new();
    for index_string in &splist_command[1..] {
        match index_string.parse::<usize>() {
            Ok(index) => {
                if index > 0 {
                    remove_index.push(index - 1);
                }
            }
            Err(_) => app.error = Some(String::from("It must be a positive integer")),
        };
    }
    if remove_index.len() > 0 {
        app.remove_play_list_by_id(remove_index);
    }
}
