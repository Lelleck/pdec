use std::fmt::Debug;

pub type OptScreen = Option<Box<dyn Screen>>;
pub trait Screen: Debug {
    fn update(&mut self, ui: &mut egui::Ui) -> Option<Box<dyn Screen>>;
}
