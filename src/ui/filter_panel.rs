use egui::Ui;
use crate::color::class_color;
use crate::star::SpectralClass;
use crate::ui::FilterState;

pub fn filter_panel(ui: &mut Ui, filters: &mut FilterState) {
    ui.heading("FILTERS");
    ui.separator();

    ui.label("Spectral Class");
    ui.horizontal(|ui| {
        if ui.button("All").clicked() {
            filters.classes = [true; 7];
        }
        if ui.button("None").clicked() {
            filters.classes = [false; 7];
        }
    });

    for (i, class) in SpectralClass::ALL.iter().enumerate() {
        ui.horizontal(|ui| {
            let color = class_color(*class);
            ui.colored_label(color, "●");
            ui.checkbox(
                &mut filters.classes[i],
                format!("{} – {}", class.name(), class.description()),
            );
        });
    }

    ui.separator();
    ui.label("Habitable Zone");
    ui.checkbox(&mut filters.hz_only, "HZ Candidates Only");

    ui.separator();
    ui.label("Luminosity (L☉)");
    ui.horizontal(|ui| {
        ui.label("Min:");
        ui.add(
            egui::DragValue::new(&mut filters.lum_min)
                .speed(0.1)
                .range(0.0_f32..=1_000_000.0_f32),
        );
    });
    ui.horizontal(|ui| {
        ui.label("Max:");
        ui.add(
            egui::DragValue::new(&mut filters.lum_max)
                .speed(0.1)
                .range(0.0_f32..=1_000_000.0_f32),
        );
    });

    ui.separator();
    ui.label("Mass (M☉)");
    ui.horizontal(|ui| {
        ui.label("Min:");
        ui.add(
            egui::DragValue::new(&mut filters.mass_min)
                .speed(0.01)
                .range(0.0_f32..=200.0_f32),
        );
    });
    ui.horizontal(|ui| {
        ui.label("Max:");
        ui.add(
            egui::DragValue::new(&mut filters.mass_max)
                .speed(0.1)
                .range(0.0_f32..=200.0_f32),
        );
    });
}
