pub mod top_bar;
pub mod filter_panel;
pub mod galaxy_canvas;
pub mod star_detail;
pub mod status_bar;
pub mod legend;

use std::collections::HashMap;
use egui::{Pos2, Vec2};
use crate::galaxy::Galaxy;
use crate::star::{SpectralClass, Star};

pub struct AppState {
    pub galaxy: GalaxyData,
    pub view: ViewState,
    pub filters: FilterState,
    pub selection: Option<usize>,
    pub ui: UiState,
}

impl Default for AppState {
    fn default() -> Self {
        let seed = rand::random::<u64>();
        let star_count = 50_000;
        let galaxy = Galaxy::generate(seed, star_count);
        let spatial_grid = SpatialGrid::build(&galaxy.stars, 200.0);
        let seed_str = seed.to_string();
        Self {
            galaxy: GalaxyData { galaxy, spatial_grid },
            view: ViewState::default(),
            filters: FilterState::default(),
            selection: None,
            ui: UiState {
                detail_panel_open: false,
                seed_input: seed_str,
                star_count_input: "50000".to_string(),
            },
        }
    }
}

impl AppState {
    pub fn regenerate(&mut self) {
        let seed = self.ui.seed_input.parse::<u64>().unwrap_or_else(|_| rand::random());
        let count = self.ui.star_count_input.parse::<usize>().unwrap_or(50_000).clamp(1_000, 1_000_000);
        let galaxy = Galaxy::generate(seed, count);
        let spatial_grid = SpatialGrid::build(&galaxy.stars, 200.0);
        self.ui.seed_input = galaxy.seed.to_string();
        self.galaxy = GalaxyData { galaxy, spatial_grid };
        self.selection = None;
        self.ui.detail_panel_open = false;
    }
}

pub struct GalaxyData {
    pub galaxy: Galaxy,
    pub spatial_grid: SpatialGrid,
}

pub struct ViewState {
    pub offset: Vec2,
    pub zoom: f32,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            zoom: 0.09,
        }
    }
}

impl ViewState {
    pub fn world_to_screen(&self, world: [f32; 2], canvas_center: Pos2) -> Pos2 {
        Pos2 {
            x: canvas_center.x + world[0] * self.zoom + self.offset.x,
            y: canvas_center.y + world[1] * self.zoom + self.offset.y,
        }
    }

    pub fn screen_to_world(&self, screen: Pos2, canvas_center: Pos2) -> [f32; 2] {
        [
            (screen.x - canvas_center.x - self.offset.x) / self.zoom,
            (screen.y - canvas_center.y - self.offset.y) / self.zoom,
        ]
    }
}

pub struct FilterState {
    pub classes: [bool; 7],
    pub hz_only: bool,
    pub lum_min: f32,
    pub lum_max: f32,
    pub mass_min: f32,
    pub mass_max: f32,
}

impl Default for FilterState {
    fn default() -> Self {
        Self {
            classes: [true; 7],
            hz_only: false,
            lum_min: 0.0,
            lum_max: 1_000_000.0,
            mass_min: 0.0,
            mass_max: 200.0,
        }
    }
}

impl FilterState {
    pub fn passes(&self, star: &Star) -> bool {
        // Check spectral class
        let class_idx = match star.class {
            SpectralClass::O => 0,
            SpectralClass::B => 1,
            SpectralClass::A => 2,
            SpectralClass::F => 3,
            SpectralClass::G => 4,
            SpectralClass::K => 5,
            SpectralClass::M => 6,
        };
        if !self.classes[class_idx] {
            return false;
        }

        // HZ only filter
        if self.hz_only && star.habitable_zone.is_none() {
            return false;
        }

        // Luminosity filter
        let lum = star.luminosity_solar as f32;
        if lum < self.lum_min || lum > self.lum_max {
            return false;
        }

        // Mass filter
        let mass = star.mass_solar as f32;
        if mass < self.mass_min || mass > self.mass_max {
            return false;
        }

        true
    }
}

pub struct UiState {
    pub detail_panel_open: bool,
    pub seed_input: String,
    pub star_count_input: String,
}

pub struct SpatialGrid {
    cells: HashMap<(i32, i32), Vec<usize>>,
    cell_size: f32,
}

impl SpatialGrid {
    pub fn build(stars: &[Star], cell_size: f32) -> Self {
        let mut cells: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
        for (i, star) in stars.iter().enumerate() {
            let cx = (star.position[0] / cell_size).floor() as i32;
            let cy = (star.position[1] / cell_size).floor() as i32;
            cells.entry((cx, cy)).or_default().push(i);
        }
        Self { cells, cell_size }
    }

    pub fn find_nearest(&self, world_pos: [f32; 2], stars: &[Star], hit_radius: f32) -> Option<usize> {
        let cx = (world_pos[0] / self.cell_size).floor() as i32;
        let cy = (world_pos[1] / self.cell_size).floor() as i32;

        let mut best_idx = None;
        let mut best_dist2 = hit_radius * hit_radius;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(indices) = self.cells.get(&(cx + dx, cy + dy)) {
                    for &idx in indices {
                        let s = &stars[idx];
                        let ddx = s.position[0] - world_pos[0];
                        let ddy = s.position[1] - world_pos[1];
                        let d2 = ddx * ddx + ddy * ddy;
                        if d2 < best_dist2 {
                            best_dist2 = d2;
                            best_idx = Some(idx);
                        }
                    }
                }
            }
        }

        best_idx
    }
}
