use rodio::Sink;

pub struct AudioPlayer {
    sounds: Vec<Option<Sink>>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let sounds: [Option<Sink>; 9] = Default::default();
        let sounds = Vec::from(sounds);
        AudioPlayer {sounds}
    }

    pub fn add_sound(&mut self, file_path: &str, index: usize) {
        let file = std::fs::File::open(file_path).unwrap();
        let (sink, _output) = rodio::Sink::new();
        let source = rodio::Decoder::new(std::io::BufReader::new(file)).unwrap();
        sink.append(source);
        self.sounds[index] = Some(sink);
    }

    pub fn play_sound(&self, index: usize) {
        if let Some(sink) = &self.sounds[index] {
            sink.set_volume(1.0);
            sink.play();
        }
    }
}
