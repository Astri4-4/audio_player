use std::{io::{self, stdout}, thread, time::Duration};
use crate::audio::{get_duration_display, get_duration_from_int};
use crossterm::{cursor::{Hide, MoveTo}, event::KeyEventKind, terminal::size};
use crossterm::style::{
    Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor
};
use crossterm::event::{self, KeyCode, KeyEvent, Event};
use crossterm::execute;

#[cfg(target_os = "windows")]
pub fn clear_terminal() {
    use std::process::Command;

    // Use the built-in "cls" command in Windows
    let _ = Command::new("cmd")
        .args(&["/C", "cls"])
        .status();
}

#[cfg(not(target_os = "windows"))]
pub fn clear_terminal() {
    use std::io::{self, Write};

    // ANSI escape code for clearing screen (works on Unix/Linux/macOS)
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

pub fn get_user_input() -> String {

    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("Unable to read user input");

    return input;

}

pub fn track_playing_display(max_time: u64, track_name: &str) {
    // █ = alt+219
    // ■ = alt + 254

    clear_terminal();

    let mut current_timestamp: u64 = 0;

    while current_timestamp < max_time {

        let mut stdout: io::Stdout = stdout();
        let terminal_size: (u16, u16) = get_term_size();
        let cols: u16 = terminal_size.0;
        let rows: u16 = terminal_size.1;

        // println!("{}", track_name);

        let col_to_print = cols/2 - track_name.len() as u16/2 ;

        execute!(
            stdout,
            MoveTo(col_to_print, rows/2),
            Print(track_name)
        ).unwrap();

        let percent:f64 = current_timestamp as f64 / max_time as f64 * 100.0;
        let mut cursor: f64 = 0.0;
        let mut to_print: String = String::new();

        // print!("{} |", get_duration_from_int(current_timestamp));

        to_print.push_str(&get_duration_from_int(current_timestamp));
        to_print.push_str(" |");

        while cursor < percent/10.0 {
            // print!(" █");
            // to_print.push_str(" █");
            to_print.push_str(" #");
            cursor += 1.0;
        }

        let rest = 10.0 - (percent/10.0);

        let mut cursor: f64 = 0.0;

        while cursor < rest {
            // print!(" ■");
            // to_print.push_str(" ■");
            to_print.push_str(" -");
            cursor += 1.0;

        }

        // println!(" | {}", get_duration_display(track_name));
        to_print.push_str("| ");
        to_print.push_str(&get_duration_display(track_name));

        to_print.push_str(" ");

        let col_to_print = (cols as f64 / 2.0) - (to_print.len() as f64 / 2.0);
        

        // 00:00 | a a a a a a a a a a a | 00:00 -> 37 char

        execute!(
            stdout,
            // MoveTo(cols / 2 - (37 / 2), rows/2+1),
            MoveTo(col_to_print as u16, rows/2+1),
            Print(to_print)
        ).unwrap();

        

        print_at_bottom(vec!["Press 'q' to exit".to_string()]);

        thread::sleep(Duration::from_secs(1));

        clear_terminal();

        current_timestamp += 1;

    }

}

pub fn get_term_size() -> (u16, u16) {

    return crossterm::terminal::size().unwrap();

}

pub fn track_list_display(files: Vec<String>) -> i32 {

    let mut action_btn: bool = false;
    let mut cursor_line: i32 = 0; 

    loop {
        let files: Vec<String> = files.clone();
        let files_clone: Vec<String> = files.clone();
        let files_len = files.len();

        let mut stdout: io::Stdout = stdout();

        let mut writing_cursor_line: i32 = 0;

        // Printing
        for file in files {
            // println!("{}", file);

            let terminal_size = get_term_size();
            let cols = terminal_size.0;
            let rows = terminal_size.1;
            let row_to_print = rows / 2 + writing_cursor_line as u16 - files_len as u16;
            let col_to_print = cols / 2 - file.len() as u16 / 2;
            let mut to_print: String = String::new();
            to_print.push_str(&file);
            to_print.push_str("\n");

            if cursor_line == writing_cursor_line {
                execute!(
                    stdout,
                    MoveTo(col_to_print, row_to_print),
                    SetForegroundColor(Color::Black),
                    SetBackgroundColor(Color::White),
                    Print(to_print),
                    ResetColor
                ).unwrap();
            } else {
                execute!(
                    stdout,
                    MoveTo(col_to_print, row_to_print),
                    SetForegroundColor(Color::White),
                    SetBackgroundColor(Color::Black),
                    Print(to_print),
                    ResetColor,
                ).unwrap();
            }

            writing_cursor_line += 1;
        }

        print_at_bottom(vec!["Press 'Ctrl+c' to exit".to_string()]);

        // Pointing
        loop {
            let files: Vec<String> = files_clone.clone();
            if event::poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(KeyEvent { code, kind, .. }) = event::read().unwrap() {
    
                    if code == KeyCode::Down && kind == KeyEventKind::Press {
                        if cursor_line < files.len() as i32 - 1{
                            cursor_line += 1;
                        }
                    } else if code == KeyCode::Up && kind == KeyEventKind::Press {
                        if cursor_line > 0 as i32 {
                            cursor_line -= 1;
                        } 
                        
                    } else if code == KeyCode::Enter && kind == KeyEventKind::Press {
                        action_btn = true;
                    }

                    if kind == KeyEventKind::Release {
                        break;
                    }

                }
            }
        }

        if action_btn == true {
            break;
        }

        // thread::sleep(Duration::from_millis(100));


        clear_terminal();

    }

    return cursor_line;

}

fn print_at_bottom(contents: Vec<String>) {

    for content in contents {
        let mut stdout = stdout();

        if let Ok((.., rows)) = size() {

            execute!(
                stdout,
                MoveTo(0, rows -1 ),
                Print(content),
                Hide
            ).unwrap();

        }

    }

}