# Starmapper — Gap Analysis & Open Items

## Status: Pre-implementation

All gaps below must be resolved before or during the implementation phase.
Items marked **BLOCKER** must be decided before writing any code in that area.

---

## 1. Cargo.toml — Missing Dependencies (BLOCKER)

No dependencies are declared. All crates below are required:

| Crate | Version | Features | Purpose |
|-------|---------|----------|---------|
| `eframe` | `0.28` | `default` | Desktop app runner |
| `egui` | `0.28` | `default` | Immediate-mode UI + Painter |
| `rand` | `0.8` | `default` | RNG for generation |
| `rand_distr` | `0.4` | `default` | Weighted class sampling, Normal dist |
| `image` | `0.25` | `png` | Headless PNG export |
| `clap` | `4` | `derive` | CLI flags (--seed, --stars, --export) |
| `serde` | `1` | `derive` | Serialization for save/load |
| `serde_json` | `1` | — | Galaxy state JSON format |
| `thiserror` | `1` | — | Typed error hierarchy |

---

## 2. Missing Documentation

### 2a. Testing Strategy
- No test plan exists for any module
- Minimum required unit tests:
  - `color.rs` — `kelvin_to_rgb` spot checks at class boundaries (3700K, 6000K, 30000K)
  - `habitable.rs` — HZ inner/outer for L=1.0 should equal 0.95/1.37 AU exactly
  - `galaxy.rs` — star count matches requested N, positions within expected galactic radius
  - `star.rs` — class weights sum to 1.0, temperature in class range

### 2b. Performance Plan
- 50,000 stars rendered per frame; no culling or batching strategy defined
- **Required decisions:**
  - View-frustum culling: skip stars outside canvas rect (world → screen transform check)
  - Spatial grid for click hit-testing: uniform grid buckets, cell size = hit radius in world units
  - Batch draw: egui Painter circles are individual calls — benchmark if `epaint::Shape::Vec` batching helps
  - LOD: at low zoom, collapse nearby stars to single dot

### 2c. Serialization Spec
- No save/load format defined
- Proposed: JSON via `serde_json`, file extension `.starmap`
- Must decide what to serialize: full `GalaxyData` (seed + star vec) or seed-only (regenerate on load)
- Seed-only is much smaller; full vec preserves any manual edits

### 2d. Config Persistence
- No mechanism to persist user preferences across sessions
- Items to persist: window size, last used seed, filter state, zoom/pan, detail panel open state
- Proposed: `~/.config/starmapper/config.json` via `serde_json`; load on startup, write on exit

### 2e. Error Handling Strategy
- No error types defined; `main.rs` has no `Result` return
- Required: a top-level `StarMapError` enum (`thiserror`) covering:
  - `GenerationError` — invalid seed, star count out of range
  - `ExportError` — file write failure, invalid path
  - `ConfigError` — malformed config file

### 2f. Build & Release
- No `[profile.release]` optimization settings in Cargo.toml
- No CI configuration
- Recommended release profile additions:
  ```toml
  [profile.release]
  opt-level = 3
  lto = true
  codegen-units = 1
  ```
- Cross-compilation targets not defined (macOS arm64, x86_64, Windows, Linux)

---

## 3. Open Design Questions

Each of these is referenced in architecture.md or ui-mockups.md but left unresolved.

### 3a. Star Naming Convention
- Mockup shows `HD-4821` but no generation logic specified
- **Options:**
  - Sequential: `HD-{id}` padded to 6 digits
  - Random: `HD-{rng(1..999999)}`
  - Procedural: combine spectral class + position hash
- **Recommendation:** `HD-{id:06}` — deterministic, unique, simple

### 3b. Luminosity → Screen Radius Formula
- Mockup says "1–3px radius scaled by luminosity" — no formula given
- **Proposed:**
  ```rust
  let radius = (1.0 + 2.0 * (star.luminosity_solar.log10() + 1.0).clamp(0.0, 1.0)) * zoom_factor;
  ```
  - M-dwarf (0.001 L☉) → ~1px, Sun (1.0 L☉) → ~2px, O-type (100,000 L☉) → ~3px
- **Needs benchmark:** test at 50k stars to confirm rendering stays above 30fps

### 3c. Hit-Testing Strategy (50k Stars)
- Brute-force O(n) per click is ~50k distance checks — likely fast enough but unverified
- **Proposed:** uniform spatial grid, cell size = 16px world-space units
  - Build grid once after generation, rebuild on filter change
  - On click: check only stars in the 3×3 cells around cursor cell
  - Reduces checks from 50k to ~10–100 per click
- **Decision needed before** `galaxy_canvas.rs` is written

### 3d. HZ Ring Dash Pattern
- Architecture says "faint dashed circle" — no specifics
- **Proposed:** 8px dash, 4px gap, stroke width 1px, color `#00FF88` at 50% opacity
- egui Painter has no native dashed circle; must approximate with arc segments or line segments

### 3e. Font & Theme
- egui ships with a default dark theme; no custom theme defined
- **Proposed defaults:**
  - Theme: egui dark (no custom)
  - Body font: 13px (egui default)
  - Monospace values (temperature, AU): `egui::TextStyle::Monospace`
  - Canvas background: `#0A0A14` (near-black with slight blue cast)

### 3f. Window Defaults & Constraints
- Not defined anywhere
- **Proposed:**
  - Default size: 1400 × 860px
  - Minimum size: 800 × 600px
  - Resizable: true
  - Title: `"Starmapper"`

---

## 4. Implementation Order (Critical Path)

```
Phase 1 — Foundation (no UI)
  [ ] Cargo.toml with all dependencies
  [ ] star.rs — SpectralClass, Star, mass/luminosity sampling
  [ ] color.rs — kelvin_to_rgb, class_color
  [ ] habitable.rs — habitable_zone(luminosity) -> HabitableZone
  [ ] galaxy.rs — Galaxy::generate(seed, n) with spiral placement

Phase 2 — Rendering Backend
  [ ] render/mod.rs — StarMapRenderer trait
  [ ] render/egui_renderer.rs — Painter-backed impl
  [ ] render/png_renderer.rs — image-crate impl

Phase 3 — App Shell
  [ ] main.rs — eframe::run_native + CLI parsing
  [ ] ui/mod.rs — AppState definition

Phase 4 — UI Panels (can parallelize within phase)
  [ ] ui/galaxy_canvas.rs — main map + pan/zoom + click
  [ ] ui/filter_panel.rs
  [ ] ui/top_bar.rs
  [ ] ui/star_detail.rs
  [ ] ui/status_bar.rs
  [ ] ui/legend.rs

Phase 5 — Export & Persistence
  [ ] --export PNG CLI path via PngRenderer
  [ ] Config persistence (~/.config/starmapper/config.json)
  [ ] Save/load galaxy (.starmap files)

Phase 6 — Polish & Performance
  [ ] Spatial grid for hit-testing
  [ ] View-frustum culling
  [ ] Unit tests for all Phase 1 modules
  [ ] Release profile tuning
```

---

## 5. Risk Register

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| egui Painter too slow for 50k circles per frame | Medium | High | Benchmark early; add LOD/culling in Phase 6 |
| Dashed HZ ring approximation looks bad | Low | Low | Test with arc segment count ≥ 64 |
| serde_json galaxy file too large at 50k stars | Low | Medium | Default to seed-only save; full vec opt-in |
| egui version API churn (0.27 → 0.28) | Medium | Medium | Pin exact versions; don't use nightly features |
| Hit-test lag at 50k with brute-force O(n) | Low | Medium | Benchmark first; spatial grid if >16ms |
