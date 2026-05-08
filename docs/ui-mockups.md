# Starmapper — UI Mockups & Interaction Spec

---

## Full Application Layout

```
╔══════════════════════════════════════════════════════════════════════════════════╗
║  ★ STARMAPPER                   Seed: [42_________] Stars: [50000] [Regenerate] ║  ← TopBar
╠══════════════╦═════════════════════════════════════════════╦═════════════════════╣
║              ║                                             ║                     ║
║  FILTERS     ║                                             ║  STAR DETAILS       ║
║              ║                                             ║                     ║
║ Spectral     ║                                             ║  HD-4821            ║
║ ─────────    ║                                             ║  ─────────────────  ║
║ [✓] O Blue   ║                                             ║  Class   G2 V       ║
║ [✓] B B-Wht  ║                                             ║  Temp    5,778 K    ║
║ [✓] A White  ║                                             ║  Lum     1.00 L☉   ║
║ [✓] F Y-Wht  ║         · · ·    ·   ·  ·   ·               ║  Mass    1.00 M☉   ║
║ [✓] G Yellow ║      ·    · · ● · ·    ·                    ║                     ║
║ [✓] K Orange ║       ·  · ·●· ◎ ·● ·  · ·                  ║  Habitable Zone     ║
║ [✓] M Red    ║     ·  ·   · ·●·  ·  ·    ·                 ║  ✓ HZ Candidate     ║
║              ║       · ·  · ○ ·  ·● · · ·                  ║  Inner   0.95 AU    ║
║ Habitable    ║      · · ·  ●·  · ·   ·                     ║  Outer   1.37 AU    ║
║ ─────────    ║           ·  · ·  ·  · ·                    ║                     ║
║ [ ] HZ Only  ║                                             ║  Position           ║
║              ║                                             ║  X  +12,430 ly      ║
║ Luminosity   ║                                             ║  Y  -8,291 ly       ║
║ ─────────    ║                                             ║                     ║
║ 0.0 ──●── ∞  ║                                             ║                     ║
║              ║                                             ║                     ║
║ Mass (M☉)   ║                                             ║                     ║
║ ─────────    ║                                             ║                     ║
║ 0.1 ──●── 60 ║                                             ║                     ║
║              ║                                             ║                     ║
║ LEGEND       ║                                             ║                     ║
║ ─────────    ║                                             ║                     ║
║ ● O  #9BB0FF ║                                             ║                     ║
║ ● B  #AABFFF ║                                             ║                     ║
║ ● A  #CAD7FF ║                                             ║                     ║
║ ● F  #F8F7FF ║                                             ║                     ║
║ ● G  #FFF4EA ║                                             ║                     ║
║ ● K  #FFD2A1 ║                                             ║                     ║
║ ● M  #FFCC6F ║                                             ║                     ║
║ ○   HZ ring  ║                                             ║                     ║
╠══════════════╩═════════════════════════════════════════════╩═════════════════════╣
║  Stars: 50,000  │  HZ Candidates: 3,241 (6.5%)  │  Zoom: 1.2×  │  [Export PNG]  ║  ← StatusBar
╚══════════════════════════════════════════════════════════════════════════════════╝
```

---

## Panel Details

### TopBar

```
╔══════════════════════════════════════════════════════════════════════════════════╗
║  ★ STARMAPPER           Seed: [____________]  Stars: [_____]  [Regenerate]  [?] ║
╚══════════════════════════════════════════════════════════════════════════════════╝
```

- **App title** (left, bold)
- **Seed input** — text field, accepts any integer; empty = random seed on generate
- **Star count input** — numeric field, 1,000–1,000,000 (validated on blur)
- **Regenerate button** — clears selection, re-runs `Galaxy::generate(seed, count)`
- **Help button `[?]`** — opens a floating key-binding reference popup

---

### FilterPanel (Left Sidebar, fixed 200px)

```
╔═══════════════╗
║ FILTERS       ║
║               ║
║ Spectral Class║
║ ─────────     ║
║ [✓] ● O       ║
║ [✓] ● B       ║
║ [✓] ● A       ║
║ [✓] ● F       ║
║ [✓] ● G       ║
║ [✓] ● K       ║
║ [✓] ● M       ║
║  [All] [None] ║
║               ║
║ Habitable     ║
║ ─────────     ║
║ [✓] HZ Only   ║
║               ║
║ Luminosity L☉ ║
║ ─────────     ║
║ Min: [0.0___] ║
║ Max: [∞_____] ║
║ 0 ────●────── ║
║               ║
║ Mass M☉       ║
║ ─────────     ║
║ Min: [0.08__] ║
║ Max: [60.0__] ║
║ ●─────────●── ║
║               ║
║ LEGEND        ║
║ ─────────     ║
║ ● O  Blue     ║
║ ● B  Bl-Wht   ║
║ ● A  White    ║
║ ● F  Yw-Wht   ║
║ ● G  Yellow   ║
║ ● K  Orange   ║
║ ● M  Red-Org  ║
║ ○   HZ ring   ║
╚═══════════════╝
```

- Spectral class checkboxes with colored dot preview; `[All]` / `[None]` shortcuts
- HZ Only toggle — hides all non-HZ stars
- Luminosity and Mass range sliders with text input fallback
- Legend always visible at bottom of panel

---

### GalaxyCanvas (CentralPanel)

```
╔═════════════════════════════════════════════════════════╗
║                      · ·  ·   ·  ·                     ║
║               ·  ·  · ·● ·  ·  ·                       ║
║          ·  ·   · ·●○·  ●·  · ·    ·                   ║
║        ·  ·  ·  ·●·  ·  ·●· · ·                        ║
║          ·  ·    · ·    ·  ·  ·                         ║
║            · · ·   ·  ·   · ·  ·                        ║
║                 ·   ·  ·   ·                            ║
║  ┌────────────────────────────────────┐                 ║
║  │ ◎  Selected star pulse highlight   │                 ║
║  │    (white outer ring, 2px)         │                 ║
║  └────────────────────────────────────┘                 ║
╚═════════════════════════════════════════════════════════╝
```

**Symbol Key:**
- `·` — regular star dot (1–3px radius scaled by luminosity)
- `●` — brighter / larger star (higher luminosity)
- `○` — HZ ring (teal-green dashed circle around a habitable-zone star)
- `◎` — selected star (white outer glow pulse animation)

**Grid:**
- Faint 10% opacity grid lines in galactic-coordinate space
- Grid spacing adapts to zoom level (doubles every 2× zoom)
- Axis labels: `+X (ly)` / `+Y (ly)` at grid intersections

**Interaction overlays:**
- Tooltip on hover: `[Class] Temp: K  Lum: L☉` (single line, 200ms delay)
- Click: selection ring appears, detail panel slides in from right
- Zoom indicator badge (bottom-right corner): `1.2×`

---

### StarDetailPanel (Right Sidebar, 220px, collapsible)

```
╔══════════════════════╗
║ STAR DETAILS      [×]║
║                      ║
║  ● HD-4821           ║
║  ──────────────────  ║
║  Class    G2 V       ║
║  Temp     5,778 K    ║
║  Luminosity          ║
║    1.00 L☉          ║
║  Mass     1.00 M☉   ║
║                      ║
║  Habitable Zone      ║
║  ──────────────────  ║
║  ✓ HZ Candidate      ║
║  Inner   0.95 AU     ║
║  Outer   1.37 AU     ║
║                      ║
║  ░░░░░░░░░░░░░░░░░░  ║  ← HZ range bar
║  ·········|·····     ║  ← Earth-equivalent
║                      ║
║  Position            ║
║  ──────────────────  ║
║  X   +12,430 ly      ║
║  Y   -8,291 ly       ║
║                      ║
║  [Center in View]    ║
╚══════════════════════╝
```

- `[×]` closes panel and clears selection
- Star color dot matches spectral class color
- HZ range bar: visual strip showing inner/outer AU range with Earth-equivalent marker
- `[Center in View]` pans and zooms the canvas to center on this star
- If no star selected: shows placeholder text "Click a star to inspect"

---

### StatusBar (Bottom, fixed 28px)

```
╔══════════════════════════════════════════════════════════════════════════════════╗
║  Stars: 50,000  │  Visible: 49,112  │  HZ Candidates: 3,241 (6.5%)  │  Zoom: 1.2×  │  [Export PNG]  ║
╚══════════════════════════════════════════════════════════════════════════════════╝
```

- **Stars** — total generated
- **Visible** — after applying current filters
- **HZ Candidates** — count + percentage of total
- **Zoom** — current scale factor
- **Export PNG** — triggers headless render at 4K resolution with current filters applied

---

## State Transitions

### Generation Flow

```
[App Launch]
     │
     ▼
AppState::default()         ← seed=random, count=50,000
     │
     ▼
Galaxy::generate(seed, n)   ← spiral placement + habitable zone calc
     │
     ▼
GalaxyData stored            ← triggers canvas redraw
```

### Selection Flow

```
[Click on GalaxyCanvas]
     │
     ├─ Hit-test: find nearest star within 8px screen-space
     │
     ├─ Star found?
     │     YES → selection = Some(star.id)
     │            detail panel opens (if closed)
     │            canvas adds selection ring
     │
     └─ No star hit?
           → selection = None
              detail panel closes
```

### Filter Change Flow

```
[Toggle checkbox / move slider in FilterPanel]
     │
     ▼
FilterState updated (immediate)
     │
     ▼
GalaxyCanvas re-evaluates visible_stars(filters)  ← no regeneration
     │
     ▼
StatusBar counts update
     │
     ▼
If selected star is now filtered out → selection = None
```

---

## Zoom & Pan Behaviour

```
World coordinate origin (0, 0) = galactic center

ViewState {
    offset: Vec2,   // screen-space pan; (0,0) = center of canvas rect
    zoom: f32,      // 1.0 = 1 light-year per N pixels (configurable constant)
}

World → Screen:
    screen_x = canvas_center.x + (world_x * zoom) + offset.x
    screen_y = canvas_center.y + (world_y * zoom) + offset.y

Screen → World (for hit-testing):
    world_x = (screen_x - canvas_center.x - offset.x) / zoom
    world_y = (screen_y - canvas_center.y - offset.y) / zoom

Zoom step: ×1.1 per scroll tick, centered on cursor
Zoom limits: 0.05× (full galaxy) to 50× (individual star region)
```

---

## Responsive Behaviour

| Window Width | Layout Change                                      |
|--------------|----------------------------------------------------|
| > 1200px     | Full layout (left panel + canvas + right panel)    |
| 800–1200px   | Right detail panel becomes bottom sheet on select  |
| < 800px      | Left filter panel collapses to icon strip          |

---

## Keyboard Shortcuts Reference

| Key        | Action                                    |
|------------|-------------------------------------------|
| `R`        | Regenerate galaxy (new random seed)       |
| `E`        | Export current view as PNG                |
| `H`        | Toggle HZ-only filter                     |
| `L`        | Toggle legend visibility                  |
| `Escape`   | Deselect / close detail panel             |
| `0`        | Reset zoom and pan to default             |
| `+` / `-`  | Zoom in / zoom out                        |
| Arrow keys | Pan canvas (10px per tick)                |
| `F`        | Frame selection (zoom to selected star)   |
| `?`        | Toggle keyboard shortcut help popup       |
