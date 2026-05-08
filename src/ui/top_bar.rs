use egui::Ui;
use crate::ui::{GalaxyData, UiState};

pub fn top_bar(
    ui: &mut Ui,
    galaxy: &GalaxyData,
    ui_state: &mut UiState,
    should_regenerate: &mut bool,
) {
    ui.horizontal(|ui| {
        ui.heading("★ STARMAPPER");
        ui.separator();
        ui.label("Seed:");
        ui.add(
            egui::TextEdit::singleline(&mut ui_state.seed_input)
                .desired_width(120.0)
                .hint_text("random"),
        );
        ui.label("Stars:");
        ui.add(
            egui::TextEdit::singleline(&mut ui_state.star_count_input)
                .desired_width(80.0),
        );
        if ui.button("Regenerate").clicked() {
            *should_regenerate = true;
        }
        ui.separator();
        ui.add(egui::Label::new(
            egui::RichText::new(format!("seed: {}", galaxy.galaxy.seed))
                .small()
                .weak(),
        ));
    });
}
