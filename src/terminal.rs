use std::{io::{self, stdout}, thread, time::Duration};
use crate::audio::{get_duration_display, get_duration_from_int};
use crossterm::{event::KeyEventKind, terminal::{size, ClearType}};
use crossterm::style::{
    Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor
};
use crossterm::{
    event::{self, KeyCode, KeyEvent, Event},
    terminal::{disable_raw_mode, enable_raw_mode},
};
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

fn get_term_size() -> Option<(u16, u16)> {

    size().ok()

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

    let mut current_timestamp = 0;

    while current_timestamp < max_time {

        println!("       {}", track_name);


        let percent:f64 = current_timestamp as f64 / max_time as f64 * 100.0;
        let mut cursor: f64 = 0.0;

        print!("{} |", get_duration_from_int(current_timestamp));

        while cursor < percent/10.0 {
            print!(" █");
            cursor += 1.0;
        }

        let rest = 10.0 - (percent/10.0);

        let mut cursor: f64 = 0.0;

        while cursor < rest {

            print!(" ■");
            cursor += 1.0;

        }

        println!(" | {}", get_duration_display(track_name));

        thread::sleep(Duration::from_secs(1));

        clear_terminal();

        current_timestamp += 1;

    }

}

pub fn track_list_display(files: Vec<String>) -> i32 {

    let mut action_btn: bool = false;
    let mut cursor_line: i32 = 0; 

    loop {
        let files: Vec<String> = files.clone();
        let files_clone: Vec<String> = files.clone();

        let mut stdout: io::Stdout = stdout();

        let mut writing_cursor_line: i32 = 0;

        for file in files {
            // println!("{}", file);

            let mut to_print: String = String::new();
            to_print.push_str(&file);
            to_print.push_str("\n");

            if cursor_line == writing_cursor_line {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Black),
                    SetBackgroundColor(Color::White),
                    Print(to_print),
                    ResetColor
                ).unwrap();
            } else {
                execute!(
                    stdout,
                    SetForegroundColor(Color::White),
                    SetBackgroundColor(Color::Black),
                    Print(to_print),
                    ResetColor,
                ).unwrap();
            }

            writing_cursor_line += 1;
        }

        println!("");

        if cursor_line == writing_cursor_line {
            execute!(
                stdout,
                SetForegroundColor(Color::Black),
                SetBackgroundColor(Color::White),
                Print("Exit"),
                ResetColor
            ).unwrap();
        } else {
            execute!(
                stdout,
                SetForegroundColor(Color::White),
                SetBackgroundColor(Color::Black),
                Print("Exit"),
                ResetColor,
            ).unwrap();
        }

        println!("");
        loop {
            let files: Vec<String> = files_clone.clone();
            if event::poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(KeyEvent { code, modifiers, kind, .. }) = event::read().unwrap() {
    
                    if code == KeyCode::Down && kind == KeyEventKind::Press {
                        if cursor_line < files.len() as i32{
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
