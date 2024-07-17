use eframe::egui;
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
    _ = eframe::run_native(
        "Pdec",
        native_options,
        Box::new(|cc| Ok(Box::new(PdecApp::new(cc)))),
    )
    .unwrap();
}

pub struct PdecApp {
    screen: Box<dyn Screen>,
}

impl PdecApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            screen: LoginScreen::boxed(),
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
            if let Some(next_screen) = self.screen.update(ui) {
                self.screen = next_screen;
            }
        });
    }
}
