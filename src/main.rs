use eframe::egui;
use log::debug;
use login::LoginScreen;
use screen::Screen;

pub mod display;
pub mod login;
pub mod screen;
pub mod utils;

/*
Features to do:
    Name players on y-axis
    Use actual team switch teams
        Option to discard team color
    Host on WebASM
    Y-Axis aspect to x-axis
*/

fn main() {
    let native_options = eframe::NativeOptions::default();
    debug!("Running application");
    _ = eframe::run_native(
        "Pdec",
        native_options,
        Box::new(|cc| Ok(Box::new(PdecApp::new(cc)))),
    )
    .unwrap();
}

pub struct PdecApp {
    current_screen: Box<dyn Screen>,
}

impl PdecApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut screen = LoginScreen::boxed();
        screen.fill_from_environment();

        Self {
            current_screen: screen,
        }
    }
}

impl eframe::App for PdecApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("version").show(ctx, |ui| {
            let version_str = env!("CARGO_PKG_VERSION");
            let info_str = format!(
                "Pdec v{} - Source Code available at {}",
                version_str, "https://github.com/Lelleck/pdec"
            );
            ui.label(info_str);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(next_screen) = self.current_screen.update(ui) {
                self.current_screen = next_screen;
            }
        });
    }
}
