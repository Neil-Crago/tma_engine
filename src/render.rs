use crate::geometry::Point;
use image::{ImageBuffer, Rgb};

/// A simple renderer to plot a Vec<Point> to a PNG image.
pub struct Renderer {
    width: u32,
    height: u32,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Renderer { width, height }
    }

    pub fn render(
        &self,
        points_with_info: &[(Point, usize)],
        filename: &str,
    ) -> Result<(), image::ImageError> {
        if points_with_info.is_empty() {
            return Ok(());
        }

        // Define a color palette.
        let colors = [
            Rgb([70, 140, 50]),   // Stem (Dark Green)
            Rgb([120, 200, 80]),  // Main leaflets (Bright Green)
            Rgb([80, 160, 200]),  // Left leaflet (Blue-ish)
            Rgb([200, 100, 150]), // Right leaflet (Pink-ish)
        ];

        // 1. Find the bounding box of the points.
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

        // Add a small margin for aesthetics
        let margin_x = (max_x - min_x) * 0.05;
        let margin_y = (max_y - min_y) * 0.05;
        min_x -= margin_x;
        max_x += margin_x;
        min_y -= margin_y;
        max_y += margin_y;

        let fractal_width = max_x - min_x;
        let fractal_height = max_y - min_y;

        // 2. Create an image buffer.
        let mut img = ImageBuffer::from_pixel(self.width, self.height, Rgb([10u8, 25, 15])); // Dark green-black background

        // 3. Map coordinates and plot points.
        // Plotting loop now uses the index to pick a color.
        for (p, index) in points_with_info {
            let px = ((p[0] - min_x) / fractal_width * (self.width - 1) as f64) as u32;
            let py = ((1.0 - (p[1] - min_y) / fractal_height) * (self.height - 1) as f64) as u32;

            if px < self.width && py < self.height {
                // Use the index to select the color
                let color = colors.get(*index).unwrap_or(&Rgb([255, 255, 255]));
                img.put_pixel(px, py, *color);
            }
        }

        // 4. Save the file.
        img.save(filename)
    }
}
