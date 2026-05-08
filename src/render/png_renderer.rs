use image::{Rgba, RgbaImage};
use std::path::PathBuf;
use crate::render::StarMapRenderer;

pub struct PngRenderer {
    image: RgbaImage,
    width: u32,
    height: u32,
    output_path: PathBuf,
    zoom: f32,
}

impl PngRenderer {
    pub fn new(width: u32, height: u32, output_path: impl Into<PathBuf>) -> Self {
        let mut image = RgbaImage::new(width, height);
        // Fill background
        for pixel in image.pixels_mut() {
            *pixel = Rgba([10, 10, 20, 255]);
        }
        Self {
            image,
            width,
            height,
            output_path: output_path.into(),
            zoom: 0.09,
        }
    }

    fn world_to_pixel(&self, world_pos: [f32; 2]) -> (i32, i32) {
        let cx = self.width as f32 / 2.0;
        let cy = self.height as f32 / 2.0;
        let px = cx + world_pos[0] * self.zoom;
        let py = cy + world_pos[1] * self.zoom;
        (px as i32, py as i32)
    }

    fn draw_circle_pixels(&mut self, cx: i32, cy: i32, radius: f32, color: [u8; 4]) {
        let r = radius.ceil() as i32;
        let r2 = radius * radius;
        for dy in -r..=r {
            for dx in -r..=r {
                if (dx * dx + dy * dy) as f32 <= r2 {
                    let px = cx + dx;
                    let py = cy + dy;
                    if px >= 0 && py >= 0 && px < self.width as i32 && py < self.height as i32 {
                        self.image.put_pixel(px as u32, py as u32, Rgba([color[0], color[1], color[2], color[3]]));
                    }
                }
            }
        }
    }

    fn draw_ring_pixels(&mut self, cx: i32, cy: i32, radius: f32, color: [u8; 4]) {
        let r = radius.ceil() as i32;
        let inner_r2 = (radius - 1.0) * (radius - 1.0);
        let outer_r2 = (radius + 1.0) * (radius + 1.0);
        for dy in -r - 2..=r + 2 {
            for dx in -r - 2..=r + 2 {
                let d2 = (dx * dx + dy * dy) as f32;
                if d2 >= inner_r2 && d2 <= outer_r2 {
                    let px = cx + dx;
                    let py = cy + dy;
                    if px >= 0 && py >= 0 && px < self.width as i32 && py < self.height as i32 {
                        self.image.put_pixel(px as u32, py as u32, Rgba([color[0], color[1], color[2], color[3]]));
                    }
                }
            }
        }
    }
}

impl StarMapRenderer for PngRenderer {
    fn draw_star(&mut self, world_pos: [f32; 2], radius: f32, color: [u8; 4]) {
        let (cx, cy) = self.world_to_pixel(world_pos);
        self.draw_circle_pixels(cx, cy, radius.max(1.0), color);
    }

    fn draw_hz_ring(&mut self, world_pos: [f32; 2]) {
        let (cx, cy) = self.world_to_pixel(world_pos);
        self.draw_ring_pixels(cx, cy, 4.0, [0, 255, 136, 100]);
    }

    fn draw_grid(&mut self, _spacing: f32) {
        // Not implemented for PNG export
    }

    fn flush(&mut self) {
        if let Err(e) = self.image.save(&self.output_path) {
            eprintln!("Failed to save PNG: {}", e);
        } else {
            println!("Exported to {:?}", self.output_path);
        }
    }
}
