use egui::Ui;
use crate::star::Star;
use crate::ui::{FilterState, ViewState};

pub fn status_bar(
    ui: &mut Ui,
    stars: &[Star],
    filters: &FilterState,
    view: &ViewState,
    should_export: &mut bool,
) {
    ui.horizontal(|ui| {
        let total = stars.len();
        let visible: usize = stars.iter().filter(|s| filters.passes(s)).count();
        let hz_count: usize = stars
            .iter()
            .filter(|s| filters.passes(s) && s.habitable_zone.is_some())
            .count();
        let pct = if visible > 0 {
            hz_count as f64 / visible as f64 * 100.0
        } else {
            0.0
        };

        ui.label(format!("Stars: {}", total));
        ui.separator();
        ui.label(format!("Visible: {}", visible));
        ui.separator();
        ui.label(format!("HZ Candidates: {} ({:.1}%)", hz_count, pct));
        ui.separator();
        ui.label(format!("Zoom: {:.2}x", view.zoom));

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Export PNG").clicked() {
                *should_export = true;
            }
        });
    });
}
