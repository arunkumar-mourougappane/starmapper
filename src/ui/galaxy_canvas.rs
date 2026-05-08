use egui::{Color32, Sense, Stroke, Ui, Vec2};
use crate::render::egui_renderer::{EguiRenderer, ViewTransform};
use crate::render::StarMapRenderer;
use crate::ui::{FilterState, GalaxyData, UiState, ViewState};

pub fn galaxy_canvas(
    ui: &mut Ui,
    galaxy: &GalaxyData,
    filters: &FilterState,
    view: &mut ViewState,
    selection: &mut Option<usize>,
    ui_state: &mut UiState,
) {
    let available = ui.available_size();
    let (rect, response) = ui.allocate_exact_size(available, Sense::click_and_drag());

    // Background
    ui.painter_at(rect).rect_filled(rect, 0.0, Color32::from_rgb(10, 10, 20));

    let center = rect.center();

    // Adaptive grid spacing: halves every time zoom doubles relative to default
    let base_spacing = 500.0_f32;
    let level = ((view.zoom / 0.09).log2()).floor() as i32;
    let spacing = (base_spacing / 2.0_f32.powi(level)).max(50.0);

    // Phase 1 — grid + stars + HZ rings via EguiRenderer
    {
        let transform = ViewTransform { offset: view.offset, zoom: view.zoom, canvas_rect: rect };
        let mut renderer = EguiRenderer::new(ui.painter_at(rect), transform);
        renderer.draw_grid(spacing);
        for star in &galaxy.galaxy.stars {
            if !filters.passes(star) { continue; }
            renderer.draw_star(star.position, star.screen_radius(), star.color);
            if star.habitable_zone.is_some() {
                renderer.draw_hz_ring(star.position);
            }
        }
    }

    // Phase 2 — selection ring + zoom badge (separate painter, after renderer is dropped)
    let overlay = ui.painter_at(rect);
    if let Some(sel_id) = *selection {
        if let Some(star) = galaxy.galaxy.stars.get(sel_id) {
            let screen_pos = view.world_to_screen(star.position, center);
            if rect.expand(10.0).contains(screen_pos) {
                overlay.circle_stroke(
                    screen_pos,
                    star.screen_radius() + 5.0,
                    Stroke::new(1.5, Color32::WHITE),
                );
            }
        }
    }

    // Pan
    if response.dragged() {
        view.offset += response.drag_delta();
    }

    // Zoom on scroll
    let scroll = ui.input(|i| i.smooth_scroll_delta.y);
    if scroll != 0.0 {
        if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
            if rect.contains(hover_pos) {
                let factor = 1.1_f32.powf(scroll / 50.0);
                let before_world = view.screen_to_world(hover_pos, center);
                view.zoom = (view.zoom * factor).clamp(0.005, 5.0);
                let after_screen = view.world_to_screen(before_world, center);
                view.offset += hover_pos - after_screen;
            }
        }
    }

    // Click to select
    if response.clicked() {
        if let Some(cursor) = response.interact_pointer_pos() {
            let world = view.screen_to_world(cursor, center);
            let hit_radius_world = 8.0 / view.zoom;
            let hit = galaxy
                .spatial_grid
                .find_nearest(world, &galaxy.galaxy.stars, hit_radius_world);
            let hit = hit.filter(|&i| filters.passes(&galaxy.galaxy.stars[i]));
            *selection = hit;
            ui_state.detail_panel_open = hit.is_some();
        }
    }

    // Zoom indicator
    overlay.text(
        rect.right_bottom() - Vec2::new(50.0, 20.0),
        egui::Align2::CENTER_CENTER,
        format!("{:.2}x", view.zoom),
        egui::FontId::monospace(12.0),
        Color32::from_rgba_unmultiplied(200, 200, 200, 180),
    );
}
