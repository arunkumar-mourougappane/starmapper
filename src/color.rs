use crate::star::SpectralClass;

pub fn kelvin_to_rgb(temp_k: f64) -> [u8; 3] {
    let t = (temp_k / 100.0).clamp(10.0, 400.0);

    // Red
    let r = if t <= 66.0 {
        255.0
    } else {
        (329.698727446 * (t - 60.0).powf(-0.1332047592)).clamp(0.0, 255.0)
    };

    // Green
    let g = if t <= 66.0 {
        (99.4708025861 * t.ln() - 161.1195681661).clamp(0.0, 255.0)
    } else {
        (288.1221695283 * (t - 60.0).powf(-0.0755148492)).clamp(0.0, 255.0)
    };

    // Blue
    let b = if t >= 66.0 {
        255.0
    } else if t <= 19.0 {
        0.0
    } else {
        (138.5177312231 * (t - 10.0).ln() - 305.0447927307).clamp(0.0, 255.0)
    };

    [r as u8, g as u8, b as u8]
}

pub fn kelvin_to_rgba(temp_k: f64) -> [u8; 4] {
    let [r, g, b] = kelvin_to_rgb(temp_k);
    [r, g, b, 255]
}

pub fn kelvin_to_color32(temp_k: f64) -> egui::Color32 {
    let [r, g, b] = kelvin_to_rgb(temp_k);
    egui::Color32::from_rgb(r, g, b)
}

pub fn class_color(class: SpectralClass) -> egui::Color32 {
    match class {
        SpectralClass::O => egui::Color32::from_rgb(0x9B, 0xB0, 0xFF),
        SpectralClass::B => egui::Color32::from_rgb(0xAA, 0xBF, 0xFF),
        SpectralClass::A => egui::Color32::from_rgb(0xCA, 0xD7, 0xFF),
        SpectralClass::F => egui::Color32::from_rgb(0xF8, 0xF7, 0xFF),
        SpectralClass::G => egui::Color32::from_rgb(0xFF, 0xF4, 0xEA),
        SpectralClass::K => egui::Color32::from_rgb(0xFF, 0xD2, 0xA1),
        SpectralClass::M => egui::Color32::from_rgb(0xFF, 0xCC, 0x6F),
    }
}
