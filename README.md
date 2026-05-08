# Starmapper

A procedural star map generator written in Rust. Generates a 2D galaxy with realistic stellar distributions, computes physical properties for each star, and identifies which stars could host liquid water via a Goldilocks zone calculation.

## Features

- **Procedural galaxy generation** — logarithmic spiral arms, Gaussian core bulge, and scattered disk stars, all seeded for reproducibility
- **Realistic stellar physics** — spectral classes O through M sampled via the Initial Mass Function (IMF), with temperatures, masses, and luminosities derived from the mass-luminosity relation
- **Kelvin → RGB color** — Tanner Helland blackbody approximation maps each star's temperature to an accurate display color
- **Goldilocks zone detection** — habitable zone inner/outer bounds computed per star using √L scaling; candidates highlighted with a teal ring
- **Interactive UI** — pan, zoom, click to inspect, filter by class/luminosity/mass, export to PNG

## Usage

```sh
# Launch the interactive GUI
cargo run --release

# Generate with a specific seed and star count
cargo run --release -- --seed 42 --stars 100000

# Headless PNG export (4K, no window)
cargo run --release -- --export output.png --seed 42 --stars 50000
```

## Controls

| Input | Action |
|-------|--------|
| Scroll wheel | Zoom in / out (cursor-anchored) |
| Left-click + drag | Pan |
| Left-click on star | Select — opens detail panel |
| `R` | Regenerate galaxy |
| `E` | Export current view as PNG |
| `H` | Toggle HZ-candidates-only filter |
| `Esc` | Deselect / close detail panel |
| `0` | Reset zoom and pan |

## Spectral Classes

| Class | Temp (K) | Color | IMF Weight |
|-------|----------|-------|-----------|
| O | 30,000–60,000 | Blue | ~0.00003% |
| B | 10,000–30,000 | Blue-white | ~0.13% |
| A | 7,500–10,000 | White | ~0.6% |
| F | 6,000–7,500 | Yellow-white | ~3% |
| G | 5,200–6,000 | Yellow | ~7.6% |
| K | 3,700–5,200 | Orange | ~12.1% |
| M | 2,400–3,700 | Red-orange | ~76.5% |

## Architecture

```
src/
  star.rs          — SpectralClass, Star, HabitableZone data types
  color.rs         — Kelvin → RGB (Tanner Helland) + per-class colors
  habitable.rs     — Goldilocks zone calculation
  galaxy.rs        — Procedural generation (spiral + bulge + scatter)
  render/
    mod.rs         — StarMapRenderer trait
    egui_renderer.rs — Interactive Painter-backed renderer
    png_renderer.rs  — Headless PNG export renderer
  ui/
    mod.rs         — AppState, ViewState, FilterState, SpatialGrid
    galaxy_canvas.rs — Main map (pan/zoom/click, adaptive grid)
    filter_panel.rs  — Class/HZ/luminosity/mass filters
    top_bar.rs       — Seed input, star count, regenerate
    star_detail.rs   — Selected star properties + HZ info
    status_bar.rs    — Live counts and export button
    legend.rs        — Spectral color legend
```

## Building

Requires Rust 1.85+ (edition 2021).

```sh
cargo build --release
```
