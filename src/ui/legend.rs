use egui::{Color32, Ui};
use crate::color::class_color;
use crate::star::SpectralClass;

pub fn spectral_legend(ui: &mut Ui) {
    ui.separator();
    ui.label("LEGEND");

    for class in SpectralClass::ALL {
        ui.horizontal(|ui| {
            let color = class_color(class);
            ui.colored_label(color, "●");
            ui.label(format!("{} – {}", class.name(), class.description()));
        });
    }

    // HZ ring indicator
    ui.horizontal(|ui| {
        ui.colored_label(Color32::from_rgba_unmultiplied(0, 255, 136, 200), "○");
        ui.label("Habitable Zone ring");
    });
}
