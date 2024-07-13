use egui::Ui;

pub fn label_input(ui: &mut Ui, label: &str, text: &mut String) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.text_edit_singleline(text);
    });
}
