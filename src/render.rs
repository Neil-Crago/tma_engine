/// Module for rendering fractal points to PNG images.
/// The renderer takes a set of points along with their associated transform
/// indices and produces a PNG image that visualizes the underlying fractal.
use crate::geometry::Point;
use image::{ImageBuffer, Rgb};

/// A simple renderer that plots an ordered set of points into a PNG image.
pub struct Renderer {
    width: u32,
    height: u32,
}

impl Renderer {
    /// Creates a renderer for a target image size.
    pub fn new(width: u32, height: u32) -> Self {
        Renderer { width, height }
    }

    /// Renders a set of points to `filename` using per-transform color mapping.
    pub fn render(
        &self,
        points_with_info: &[(Point, usize)],
        filename: &str,
    ) -> Result<(), image::ImageError> {
        if points_with_info.is_empty() {
            return Ok(());
        }

        let colors = [
            Rgb([70, 140, 50]),
            Rgb([120, 200, 80]),
            Rgb([80, 160, 200]),
            Rgb([200, 100, 150]),
        ];

        let mut min_x = f64::MAX;
        let mut max_x = f64::MIN;
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;

        for (p, _) in points_with_info {
            min_x = min_x.min(p[0]);
            max_x = max_x.max(p[0]);
            min_y = min_y.min(p[1]);
            max_y = max_y.max(p[1]);
        }

        let fractal_width = (max_x - min_x).max(1.0);
        let fractal_height = (max_y - min_y).max(1.0);

        let margin_x = fractal_width * 0.05;
        let margin_y = fractal_height * 0.05;
        let min_x = min_x - margin_x;
        let max_x = max_x + margin_x;
        let min_y = min_y - margin_y;
        let max_y = max_y + margin_y;

        let bounds_width = max_x - min_x;
        let bounds_height = max_y - min_y;
        let mut img = ImageBuffer::from_pixel(self.width, self.height, Rgb([10u8, 25, 15]));

        for (p, index) in points_with_info {
            let px = ((p[0] - min_x) / bounds_width * (self.width.saturating_sub(1) as f64)) as u32;
            let py = ((1.0 - (p[1] - min_y) / bounds_height)
                * (self.height.saturating_sub(1) as f64)) as u32;

            if px < self.width && py < self.height {
                let color = colors.get(*index).copied().unwrap_or(Rgb([255, 255, 255]));
                img.put_pixel(px, py, color);
            }
        }

        img.save(filename)
    }
}
