use std::thread;
use rodio::{OutputStream, Sink};
use terminal::clear_terminal;
use crossterm::event::{self, Event, KeyEvent, KeyCode};

pub mod file;
pub mod audio;
pub mod terminal;

fn main() {

    clear_terminal();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let files = file::get_all_music();

    let choice = terminal::track_list_display(files);

    let files = file::get_all_music();
    thread::spawn(move || audio::play_audio(&files[choice as usize], sink));

    let files: Vec<String> = file::get_all_music();
    let choose_track_duration = audio::get_duration(&files[choice as usize]);

    thread::spawn(move || terminal::track_playing_display(choose_track_duration.0, &files[choice as usize]));

    loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            
            if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {

                if code == KeyCode::Char('q') {
                    break;
                }

            }

        }
    }

}
