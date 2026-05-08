mod star;
mod color;
mod habitable;
mod galaxy;
mod render;
mod ui;

use std::path::PathBuf;

use clap::Parser;
use eframe::egui;
use egui::Key;

use crate::galaxy::Galaxy;
use crate::render::StarMapRenderer;
use crate::render::png_renderer::PngRenderer;
use crate::ui::{AppState, FilterState};
use crate::ui::top_bar::top_bar;
use crate::ui::filter_panel::filter_panel;
use crate::ui::galaxy_canvas::galaxy_canvas;
use crate::ui::star_detail::star_detail_panel;
use crate::ui::status_bar::status_bar;
use crate::ui::legend::spectral_legend;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(long)]
    seed: Option<u64>,
    #[arg(long, default_value = "50000")]
    stars: usize,
    #[arg(long)]
    export: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(ref path) = cli.export {
        run_headless_export(cli.seed, cli.stars, path.clone());
        return;
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Starmapper")
            .with_inner_size([1400.0, 860.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    let seed = cli.seed;
    eframe::run_native(
        "Starmapper",
        options,
        Box::new(move |_cc| {
            let mut state = AppState::default();
            if let Some(s) = seed {
                state.ui.seed_input = s.to_string();
                state.regenerate();
            }
            Ok(Box::new(StarMapperApp { state }))
        }),
    )
    .unwrap();
}

struct StarMapperApp {
    state: AppState,
}

impl eframe::App for StarMapperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Collect keyboard events
        let (key_r, key_e, key_h, key_esc, key_0) = ctx.input(|i| {
            (
                i.key_pressed(Key::R),
                i.key_pressed(Key::E),
                i.key_pressed(Key::H),
                i.key_pressed(Key::Escape),
                i.key_pressed(Key::Num0),
            )
        });

        if key_h {
            self.state.filters.hz_only = !self.state.filters.hz_only;
        }
        if key_esc {
            self.state.selection = None;
            self.state.ui.detail_panel_open = false;
        }
        if key_0 {
            self.state.view = crate::ui::ViewState::default();
        }

        let mut should_regenerate = key_r;
        let mut should_export = key_e;

        // 2. Show panels
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            top_bar(ui, &self.state.galaxy, &mut self.state.ui, &mut should_regenerate);
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            status_bar(
                ui,
                &self.state.galaxy.galaxy.stars,
                &self.state.filters,
                &self.state.view,
                &mut should_export,
            );
        });

        egui::SidePanel::left("filter_panel")
            .exact_width(220.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    filter_panel(ui, &mut self.state.filters);
                    spectral_legend(ui);
                });
            });

        if self.state.ui.detail_panel_open || self.state.selection.is_some() {
            let selected_star = self
                .state
                .selection
                .map(|idx| &self.state.galaxy.galaxy.stars[idx]);
            egui::SidePanel::right("detail_panel")
                .exact_width(240.0)
                .show(ctx, |ui| {
                    star_detail_panel(
                        ui,
                        selected_star,
                        &mut self.state.ui.detail_panel_open,
                        &mut self.state.view,
                        &mut self.state.selection,
                    );
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            galaxy_canvas(
                ui,
                &self.state.galaxy,
                &self.state.filters,
                &mut self.state.view,
                &mut self.state.selection,
                &mut self.state.ui,
            );
        });

        // 3. Apply deferred actions
        if should_regenerate {
            self.state.regenerate();
        }
        if should_export {
            self.export_png();
        }
    }
}

impl StarMapperApp {
    fn export_png(&self) {
        let path = "starmapper_export.png";
        let mut renderer = PngRenderer::new(3840, 2160, path);
        let filters = FilterState::default();
        for star in &self.state.galaxy.galaxy.stars {
            if filters.passes(star) {
                renderer.draw_star(star.position, star.screen_radius(), star.color);
                if star.habitable_zone.is_some() {
                    renderer.draw_hz_ring(star.position);
                }
            }
        }
        renderer.flush();
    }
}

fn run_headless_export(seed: Option<u64>, star_count: usize, path: PathBuf) {
    let seed = seed.unwrap_or_else(rand::random);
    let galaxy = Galaxy::generate(seed, star_count);
    let mut renderer = PngRenderer::new(3840, 2160, path);
    let filters = FilterState::default();
    for star in &galaxy.stars {
        if filters.passes(star) {
            renderer.draw_star(star.position, star.screen_radius(), star.color);
            if star.habitable_zone.is_some() {
                renderer.draw_hz_ring(star.position);
            }
        }
    }
    renderer.flush();
}
