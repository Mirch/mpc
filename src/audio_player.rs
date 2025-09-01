use std::{fs::File, thread};
use std::io::BufReader;

pub struct AudioPlayer {
    sounds: Vec<Option<File>>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let sounds: [Option<File>; 9] = Default::default();
        let sounds = Vec::from(sounds);
        AudioPlayer {sounds}
    }

    pub fn add_sound(&mut self, file_path: &str, index: usize) {
        let file = std::fs::File::open(file_path).unwrap();
        self.sounds[index] = Some(file);
    }

    pub fn play_sound(&self, index: usize) {
        if self.sounds[index].is_none() {
            return;
        }

        let file = self.sounds[index].as_ref().unwrap().try_clone().unwrap();
        let file = BufReader::new(file);
        thread::spawn(|| {
            let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
                .expect("open default audio stream");
            let _sink = rodio::play(&stream_handle.mixer(), file).unwrap();
            thread::sleep(std::time::Duration::from_secs(3));
        });
    }
}
