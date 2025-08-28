use eframe::egui;
use std::thread;
use std::time::Duration;

mod audio_player;


fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "MPC",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

struct MyApp {
    player: audio_player::AudioPlayer,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut player = audio_player::AudioPlayer::new();
        player.add_sound("audio/test.mp3", 0);
        Self {
            player
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("MPC");
            for i in 1..=3 {
                ui.horizontal(|ui| {
                    for j in 1..=3 {
                        let index = 3 * (i - 1) + j;
                        if ui.button(format!("\t\t\t\n\t\t\t {index} \n\t\t\t\n")).clicked() {
                            self.player.play_sound(0);
                            thread::sleep(Duration::from_millis(1500));
                        }
                    }
                });
            }
        });

    }
}
