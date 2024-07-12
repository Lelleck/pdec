use eframe::egui;
use log::{debug, Log};
use login::LoginScreen;
use screen::Screen;

pub mod display;
pub mod login;
pub mod screen;

fn main() {
    let native_options = eframe::NativeOptions::default();
    debug!("Running application");
    _ = eframe::run_native(
        "Sniping Analyser",
        native_options,
        Box::new(|cc| Ok(Box::new(EguiApp::new(cc)))),
    )
    .unwrap();
}

pub struct EguiApp {
    current_screen: Box<dyn Screen>,
}

impl EguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            current_screen: LoginScreen::boxed(),
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(next_screen) = self.current_screen.update(ui) {
                debug!("Switching screen to {:?}", next_screen);
                self.current_screen = next_screen;
            }
        });
    }
}
