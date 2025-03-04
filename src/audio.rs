use rodio::{Decoder, Sink};
use std::fs::File;
use std::io::BufReader;
use mp3_duration::from_path;

pub fn play_audio(file_path: &str, sink: Sink) {

    let mut path: String = "./musics/".to_string();
    path.push_str(file_path);
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();

    sink.append(source);
    sink.set_volume(0.1);
    sink.sleep_until_end();

    println!("Still working !");

}

pub fn get_duration(file_path: &str) -> (u64, u64, String) {

    let mut path: String = "./musics/".to_string();
    path.push_str(file_path);

    let audio_len_raw: u64 = from_path(path).unwrap().as_secs();
    let audio_len_min: u64 = audio_len_raw/60;
    let audio_modulo_raw = audio_len_raw % 60;
    let mut audio_modulo_str: String = audio_modulo_raw.to_string();

    if audio_modulo_str.len() < 2 {
        audio_modulo_str.insert(0, '0');
    }

    let result = (audio_len_raw, audio_len_min, audio_modulo_str);

    return result;
}

pub fn get_duration_display(file_path: &str) -> String{
    let mut path: String = "./musics/".to_string();
    path.push_str(file_path);

    let audio_len_raw: u64 = from_path(path).unwrap().as_secs();
    let audio_len_min: u64 = audio_len_raw/60;
    let audio_modulo_raw: u64 = audio_len_raw % 60;
    let mut audio_modulo_str: String = audio_modulo_raw.to_string();

    if audio_modulo_str.len() < 2 {
        audio_modulo_str.insert(0, '0');
    }

    let mut result_str: String = String::new();

    result_str.push_str(&audio_len_min.to_string().as_str());
    result_str.push_str(":");
    result_str.push_str(&audio_modulo_str);

    return result_str;

}

pub fn get_duration_from_int(timestamp: u64) -> String {

    let audio_len_min: u64 = timestamp/60;
    let audio_modulo_raw: u64 = timestamp % 60;
    let mut audio_modulo_str: String = audio_modulo_raw.to_string();

    if audio_modulo_str.len() < 2 {
        audio_modulo_str.insert(0, '0');
    }

    let mut result_str: String = String::new();

    result_str.push_str(&audio_len_min.to_string().as_str());
    result_str.push_str(":");
    result_str.push_str(&audio_modulo_str);

    return result_str;

}