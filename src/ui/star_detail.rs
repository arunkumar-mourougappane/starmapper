use egui::{Color32, Grid, RichText, Ui, Vec2};
use crate::color::kelvin_to_color32;
use crate::star::Star;
use crate::ui::ViewState;

pub fn star_detail_panel(
    ui: &mut Ui,
    star: Option<&Star>,
    open: &mut bool,
    view: &mut ViewState,
    selection: &mut Option<usize>,
) {
    ui.horizontal(|ui| {
        ui.heading("STAR DETAILS");
        if ui.button("×").clicked() {
            *open = false;
            *selection = None;
        }
    });
    ui.separator();

    let star = match star {
        Some(s) => s,
        None => {
            ui.label("Click a star to inspect.");
            return;
        }
    };

    // Star name with colored dot
    ui.horizontal(|ui| {
        let dot_color = kelvin_to_color32(star.temperature_k);
        ui.colored_label(dot_color, "●");
        ui.label(RichText::new(star.name()).strong());
    });
    ui.separator();

    Grid::new("star_detail_grid")
        .num_columns(2)
        .spacing([8.0, 4.0])
        .show(ui, |ui| {
            ui.label("Class");
            ui.label(star.class.name());
            ui.end_row();

            ui.label("Temp");
            ui.label(format!("{:.0} K", star.temperature_k));
            ui.end_row();

            ui.label("Lum");
            ui.label(format!("{:.4} L☉", star.luminosity_solar));
            ui.end_row();

            ui.label("Mass");
            ui.label(format!("{:.3} M☉", star.mass_solar));
            ui.end_row();

            ui.label("Pos");
            ui.label(format!("({:.0}, {:.0})", star.position[0], star.position[1]));
            ui.end_row();
        });

    ui.separator();
    ui.label("Habitable Zone");

    if let Some(hz) = &star.habitable_zone {
        ui.colored_label(Color32::from_rgb(0, 220, 100), "✓ HZ Candidate");
        Grid::new("hz_grid")
            .num_columns(2)
            .spacing([8.0, 4.0])
            .show(ui, |ui| {
                ui.label("Inner");
                ui.label(format!("{:.3} AU", hz.inner_au));
                ui.end_row();

                ui.label("Outer");
                ui.label(format!("{:.3} AU", hz.outer_au));
                ui.end_row();
            });

        // HZ bar
        let bar_rect = ui.allocate_space(Vec2::new(ui.available_width(), 12.0)).1;
        ui.painter().rect_filled(
            bar_rect,
            3.0,
            Color32::from_rgba_unmultiplied(0, 255, 136, 60),
        );
    } else {
        ui.colored_label(Color32::from_rgb(220, 60, 60), "✗ Outside HZ");
    }

    ui.separator();
    if ui.button("Center in View").clicked() {
        view.offset = -Vec2::new(
            star.position[0] * view.zoom,
            star.position[1] * view.zoom,
        );
    }
}
