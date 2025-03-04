use eframe::egui;
use eframe::*;
use rodio::*;
use std::{fs::File, io::BufReader, sync::{Arc, Mutex}};

struct AudioApp {
    sink: Arc<Mutex<Option<Sink>>>, // Audio control
}

impl AudioApp {
    fn new() -> Self {
        Self {
            sink: Arc::new(Mutex::new(None)),
        }
    }
}

impl eframe::App for AudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Audio Player 🎵");

            if ui.button("Play").clicked() {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let sink = Sink::try_new(&stream_handle).unwrap();

                let file = BufReader::new(File::open("audio.mp3").unwrap());
                let source = Decoder::new(file).unwrap();
                sink.append(source);
                sink.set_volume(0.5); // Start at 50% volume

                *self.sink.lock().unwrap() = Some(sink);
            }

            if ui.button("Increase Volume").clicked() {
                if let Some(sink) = &*self.sink.lock().unwrap() {
                    sink.set_volume((sink.volume() + 0.1).min(1.0));
                }
            }

            if ui.button("Decrease Volume").clicked() {
                if let Some(sink) = &*self.sink.lock().unwrap() {
                    sink.set_volume((sink.volume() - 0.1).max(0.0));
                }
            }

            if ui.button("Stop").clicked() {
                if let Some(sink) = self.sink.lock().unwrap().take() {
                    sink.stop();
                }
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Audio Player",
        options,
        Box::new(|_cc| Box::new(AudioApp::new())),
    ).unwrap();
}
