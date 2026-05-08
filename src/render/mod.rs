pub trait StarMapRenderer {
    fn draw_star(&mut self, world_pos: [f32; 2], radius: f32, color: [u8; 4]);
    fn draw_hz_ring(&mut self, world_pos: [f32; 2]);
    fn draw_grid(&mut self, spacing: f32);
    fn flush(&mut self);
}

pub mod egui_renderer;
pub mod png_renderer;
