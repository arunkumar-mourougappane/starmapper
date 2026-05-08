# Starmapper — Architecture Documentation

## Overview

Starmapper is a procedural star map generator written in Rust. It generates a 2D galaxy map, assigns
stars spectral classifications (O, B, A, F, G, K, M), computes their physical properties (temperature,
luminosity, color), and identifies which stars could host liquid water via a Goldilocks zone calculation.
The interactive UI is built with `egui`/`eframe`; an optional headless PNG export path shares the same
rendering trait.

---

## Technology Stack

| Layer       | Crate(s)                     | Purpose                                      |
|-------------|------------------------------|----------------------------------------------|
| GUI         | `egui` + `eframe`            | Immediate-mode desktop UI, custom Painter API |
| Rendering   | `egui::Painter` / `image`    | Star canvas (interactive) and PNG export      |
| Random      | `rand` + `rand_distr`        | Weighted class sampling, spiral placement     |
| CLI/Config  | `clap`                       | Seed, star count, output flags                |
| Math        | `std::f64`                   | Blackbody formulas, spiral geometry           |

---

## Directory Structure

```
starmapper/
├── Cargo.toml
├── src/
│   ├── main.rs                  # eframe::run_native entrypoint + CLI parsing
│   │
│   ├── star.rs                  # Star struct, SpectralClass enum, luminosity
│   ├── galaxy.rs                # Galaxy generation, spiral arm placement
│   ├── habitable.rs             # Goldilocks zone calculation
│   ├── color.rs                 # Kelvin → RGB/Color32 conversion
│   │
│   ├── ui/
│   │   ├── mod.rs               # AppState, re-exports all ui submodules
│   │   ├── top_bar.rs           # TopBar widget function
│   │   ├── filter_panel.rs      # FilterPanel widget fn + FilterState
│   │   ├── galaxy_canvas.rs     # GalaxyCanvas widget fn + ViewState
│   │   ├── star_detail.rs       # StarDetailPanel widget fn
│   │   ├── status_bar.rs        # StatusBar widget fn
│   │   └── legend.rs            # Spectral class color legend
│   │
│   └── render/
│       ├── mod.rs               # StarMapRenderer trait definition
│       ├── egui_renderer.rs     # EguiRenderer (Painter-backed)
│       └── png_renderer.rs      # PngRenderer (image crate)
│
└── docs/
    ├── architecture.md          # This file
    └── ui-mockups.md            # ASCII UI mockups and interaction spec
```

---

## Core Data Model

### `SpectralClass` Enum

```rust
pub enum SpectralClass { O, B, A, F, G, K, M }
```

Weighted distribution matches the Initial Mass Function (IMF):

| Class | Weight  | Temp Range (K)  |
|-------|---------|-----------------|
| O     | 0.00003 | 30,000–60,000   |
| B     | 0.13    | 10,000–30,000   |
| A     | 0.6     | 7,500–10,000    |
| F     | 3.0     | 6,000–7,500     |
| G     | 7.6     | 5,200–6,000     |
| K     | 12.1    | 3,700–5,200     |
| M     | 76.45   | 2,400–3,700     |

### `Star` Struct

```rust
pub struct Star {
    pub id: usize,
    pub class: SpectralClass,
    pub temperature_k: f64,     // sampled uniformly within class range
    pub luminosity_solar: f64,  // derived via Stefan-Boltzmann + mass relation
    pub mass_solar: f64,        // sampled from class mass range
    pub position: [f32; 2],     // world-space coords (galactic plane, AU scale)
    pub color: [u8; 4],         // RGBA from Kelvin→RGB conversion
    pub habitable_zone: Option<HabitableZone>,
}

pub struct HabitableZone {
    pub inner_au: f64,
    pub outer_au: f64,
}
```

---

## Module Responsibilities

### `star.rs`
- Defines `SpectralClass`, `Star`, `HabitableZone`
- Provides `Star::generate(rng, class)` — samples temperature and mass, computes luminosity and color
- No I/O or UI dependencies

### `galaxy.rs`
- `Galaxy::generate(seed, star_count) -> Vec<Star>`
- Logarithmic spiral placement: `r = a × e^(b×θ)` across 2–4 arms
- Gaussian scatter perpendicular to arm axis
- Central bulge: spherical overdensity with Gaussian density falloff
- Returns `Vec<Star>` with world positions assigned

### `habitable.rs`
- `fn habitable_zone(luminosity_solar: f64) -> HabitableZone`
- Inner edge: `0.95 × sqrt(L)` AU
- Outer edge: `1.37 × sqrt(L)` AU
- Called once per star during generation; result stored on `Star`

### `color.rs`
- `fn kelvin_to_rgb(temp_k: f64) -> [u8; 3]`
- Implements Tanner Helland's piecewise polynomial approximation
- Accurate to ±10 RGB units over 1,000–40,000 K range
- Separate `fn class_color(class: SpectralClass) -> egui::Color32` for legend rendering

### `render/mod.rs` — `StarMapRenderer` Trait

```rust
pub trait StarMapRenderer {
    fn draw_star(&mut self, world_pos: [f32; 2], radius: f32, color: [u8; 4]);
    fn draw_hz_ring(&mut self, world_pos: [f32; 2], inner_au: f32, outer_au: f32);
    fn draw_grid(&mut self, spacing: f32);
    fn flush(&mut self);  // PNG: write file; egui: no-op
}
```

`GalaxyCanvas` depends only on `StarMapRenderer`. It never imports `egui` or `image` directly,
keeping canvas logic backend-agnostic and fully unit-testable.

### `render/egui_renderer.rs`
- Wraps `egui::Painter` + `ViewState` (pan/zoom transform)
- Implements world → screen coordinate projection
- `draw_hz_ring` renders as a faint dashed circle around the star dot

### `render/png_renderer.rs`
- Wraps `image::RgbaImage`
- Same world → pixel projection using a fixed viewport rect
- `flush()` writes PNG to disk
- Used by headless export (`--export` CLI flag)

---

## UI State Architecture

### `AppState` (owned by `eframe::App`)

```rust
pub struct AppState {
    pub galaxy: GalaxyData,
    pub view: ViewState,
    pub filters: FilterState,
    pub selection: Option<usize>,   // index into galaxy.stars
    pub ui: UiState,
}

pub struct GalaxyData {
    pub stars: Vec<Star>,
    pub seed: u64,
    pub star_count: usize,
}

pub struct ViewState {
    pub offset: egui::Vec2,   // pan in screen-space pixels
    pub zoom: f32,            // scale factor (1.0 = default)
}

pub struct FilterState {
    pub classes: [bool; 7],   // O B A F G K M — all true by default
    pub hz_only: bool,
    pub lum_range: (f32, f32),
    pub mass_range: (f32, f32),
}

pub struct UiState {
    pub detail_panel_open: bool,
    pub legend_open: bool,
    pub seed_input: String,   // staging buffer for text input
}
```

---

## Widget Function Signatures

Each panel is a free function that takes `&mut egui::Ui` and the relevant slice of `AppState`:

```rust
// ui/top_bar.rs
pub fn top_bar(ui: &mut egui::Ui, galaxy: &mut GalaxyData, ui_state: &mut UiState);

// ui/filter_panel.rs
pub fn filter_panel(ui: &mut egui::Ui, filters: &mut FilterState);

// ui/galaxy_canvas.rs
pub fn galaxy_canvas(
    ui: &mut egui::Ui,
    stars: &[Star],
    filters: &FilterState,
    view: &mut ViewState,
    selection: &mut Option<usize>,
);

// ui/star_detail.rs
pub fn star_detail_panel(ui: &mut egui::Ui, star: Option<&Star>, open: &mut bool);

// ui/status_bar.rs
pub fn status_bar(ui: &mut egui::Ui, stars: &[Star], filters: &FilterState, view: &ViewState);

// ui/legend.rs
pub fn spectral_legend(ui: &mut egui::Ui);
```

No widget function holds state — all mutable state lives in `AppState`. This makes each widget
independently testable and freely reorderable in the layout.

---

## Rendering Pipeline (Interactive Path)

```
eframe::App::update()
  │
  ├─ TopBottomPanel::top   → top_bar()
  ├─ TopBottomPanel::bottom→ status_bar()
  ├─ SidePanel::left       → filter_panel() + spectral_legend()
  ├─ SidePanel::right      → star_detail_panel()   [if selection.is_some()]
  └─ CentralPanel          → galaxy_canvas()
                                │
                                ├─ allocate Rect, get Painter
                                ├─ construct EguiRenderer { painter, view }
                                ├─ renderer.draw_grid()
                                ├─ for star in visible_stars(filters):
                                │    renderer.draw_star(star.position, radius, star.color)
                                │    if star.habitable_zone.is_some() && hz_visible:
                                │        renderer.draw_hz_ring(...)
                                └─ handle pointer events (click, drag, scroll)
```

---

## Interaction Specification

| Input               | Effect                                           |
|---------------------|--------------------------------------------------|
| Scroll wheel        | Zoom in/out centered on cursor position          |
| Left-click + drag   | Pan the galaxy view                              |
| Left-click on star  | Select star → open StarDetailPanel               |
| Double-click star   | Zoom 2× centered on that star                    |
| Click empty space   | Deselect (clear selection)                       |
| `R` key / button    | Regenerate galaxy with new random seed           |
| `E` key / button    | Export current filtered view as PNG              |
| `H` toggle          | Toggle: show all stars / HZ candidates only      |
| `L` toggle          | Toggle spectral class legend overlay             |
| `Escape`            | Close detail panel / reset selection             |

---

## Headless Export Path (PNG)

```
$ starmapper --export output.png --seed 42 --stars 100000
```

1. `main.rs` detects `--export` flag, skips `eframe::run_native`
2. Calls `Galaxy::generate(seed, star_count)`
3. Constructs `PngRenderer { image, viewport }`
4. Runs same draw loop as `galaxy_canvas` but against `PngRenderer`
5. Calls `renderer.flush()` → writes PNG

The `StarMapRenderer` trait ensures zero duplication between interactive and export paths.

---

## Spectral Class Color Reference

| Class | Hex Color | Description    |
|-------|-----------|----------------|
| O     | `#9BB0FF` | Blue           |
| B     | `#AABFFF` | Blue-white     |
| A     | `#CAD7FF` | White          |
| F     | `#F8F7FF` | Yellow-white   |
| G     | `#FFF4EA` | Yellow (Sun)   |
| K     | `#FFD2A1` | Orange         |
| M     | `#FFCC6F` | Red-orange     |

HZ candidate ring color: `#00FF88` (teal-green glow), drawn at 60% opacity.

---

## Key Algorithms

### Logarithmic Spiral Star Placement

```rust
// For arm i of N arms:
let theta_offset = (i as f64) * (2.0 * PI / N as f64);
let theta = rng.sample(Uniform::new(0.0, 4.0 * PI));
let r = a * f64::exp(b * theta) + rng.sample(Normal::new(0.0, scatter));
let x = r * f64::cos(theta + theta_offset);
let y = r * f64::sin(theta + theta_offset);
```

### Blackbody K → RGB (Tanner Helland)

```rust
let t = temp_k / 100.0;
let r = if t <= 66.0 { 255 } else { (329.698727446 * (t - 60.0).powf(-0.1332047592)).clamp(0.0, 255.0) as u8 };
let g = if t <= 66.0 {
    (99.4708025861 * t.ln() - 161.1195681661).clamp(0.0, 255.0) as u8
} else {
    (288.1221695283 * (t - 60.0).powf(-0.0755148492)).clamp(0.0, 255.0) as u8
};
let b = if t >= 66.0 { 255 } else if t <= 19.0 { 0 } else {
    (138.5177312231 * (t - 10.0).ln() - 305.0447927307).clamp(0.0, 255.0) as u8
};
```

### Habitable Zone

```rust
pub fn habitable_zone(luminosity_solar: f64) -> HabitableZone {
    let sqrt_l = luminosity_solar.sqrt();
    HabitableZone {
        inner_au: 0.95 * sqrt_l,
        outer_au: 1.37 * sqrt_l,
    }
}
```
