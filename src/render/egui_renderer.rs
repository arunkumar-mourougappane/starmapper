use egui::{Color32, Painter, Pos2, Rect, Stroke, Vec2};
use crate::render::StarMapRenderer;

pub struct ViewTransform {
    pub offset: Vec2,
    pub zoom: f32,
    pub canvas_rect: Rect,
}

impl ViewTransform {
    pub fn world_to_screen(&self, world: [f32; 2]) -> Pos2 {
        let center = self.canvas_rect.center();
        Pos2 {
            x: center.x + world[0] * self.zoom + self.offset.x,
            y: center.y + world[1] * self.zoom + self.offset.y,
        }
    }

    pub fn in_view(&self, world: [f32; 2], radius: f32) -> bool {
        let screen = self.world_to_screen(world);
        self.canvas_rect.expand(radius).contains(screen)
    }
}

pub struct EguiRenderer {
    painter: Painter,
    transform: ViewTransform,
}

impl EguiRenderer {
    pub fn new(painter: Painter, transform: ViewTransform) -> Self {
        Self { painter, transform }
    }
}

impl StarMapRenderer for EguiRenderer {
    fn draw_star(&mut self, world_pos: [f32; 2], radius: f32, color: [u8; 4]) {
        if !self.transform.in_view(world_pos, radius) {
            return;
        }
        let pos = self.transform.world_to_screen(world_pos);
        let c = Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        self.painter.circle_filled(pos, radius, c);
    }

    fn draw_hz_ring(&mut self, world_pos: [f32; 2]) {
        let screen_pos = self.transform.world_to_screen(world_pos);
        let ring_color = Color32::from_rgba_unmultiplied(0, 255, 136, 100);
        self.painter
            .circle_stroke(screen_pos, 6.0, Stroke::new(1.0, ring_color));
    }

    fn draw_grid(&mut self, spacing: f32) {
        let rect = self.transform.canvas_rect;
        let faint = Color32::from_rgba_unmultiplied(255, 255, 255, 12);

        // Determine world-space bounds visible on screen
        let center = rect.center();
        let world_left =
            (rect.left() - center.x - self.transform.offset.x) / self.transform.zoom;
        let world_right =
            (rect.right() - center.x - self.transform.offset.x) / self.transform.zoom;
        let world_top =
            (rect.top() - center.y - self.transform.offset.y) / self.transform.zoom;
        let world_bottom =
            (rect.bottom() - center.y - self.transform.offset.y) / self.transform.zoom;

        // Vertical lines
        let x_start = (world_left / spacing).floor() as i32;
        let x_end = (world_right / spacing).ceil() as i32;
        for xi in x_start..=x_end {
            let wx = xi as f32 * spacing;
            let sx = center.x + wx * self.transform.zoom + self.transform.offset.x;
            self.painter.line_segment(
                [Pos2::new(sx, rect.top()), Pos2::new(sx, rect.bottom())],
                Stroke::new(1.0, faint),
            );
        }

        // Horizontal lines
        let y_start = (world_top / spacing).floor() as i32;
        let y_end = (world_bottom / spacing).ceil() as i32;
        for yi in y_start..=y_end {
            let wy = yi as f32 * spacing;
            let sy = center.y + wy * self.transform.zoom + self.transform.offset.y;
            self.painter.line_segment(
                [Pos2::new(rect.left(), sy), Pos2::new(rect.right(), sy)],
                Stroke::new(1.0, faint),
            );
        }
    }

    fn flush(&mut self) {
        // no-op for egui
    }
}
