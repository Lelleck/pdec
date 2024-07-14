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
        let mut screen = LoginScreen::boxed();
        screen.fill_from_environment();

        Self {
            current_screen: screen,
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("disclaimer").show(ctx, |ui| {
            ui.heading("Not fit for analysis... Uses randomized data.");
        });

        egui::TopBottomPanel::bottom("version").show(ctx, |ui| {
            let version_str = env!("CARGO_PKG_VERSION");
            let info_str = format!(
                "Sniping Analyser v{} - Source Code available at {}",
                version_str, "NOT AVAILABLE"
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
