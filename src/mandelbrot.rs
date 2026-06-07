//! Mandelbrot set computation.

use crate::escape::{self, EscapeResult};

/// Mandelbrot set evaluator.
///
/// Provides methods to test membership and compute iteration counts
/// for points in the complex plane.
pub struct Mandelbrot {
    /// Maximum iterations before classifying a point as "inside".
    pub max_iter: u32,
    /// Squared bailout radius (default 4.0).
    pub bailout_sq: f64,
}

impl Default for Mandelbrot {
    fn default() -> Self {
        Self { max_iter: 256, bailout_sq: 4.0 }
    }
}

impl Mandelbrot {
    /// Create with default settings (256 max iterations, bailout radius 2).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum iterations.
    pub fn with_max_iter(mut self, max_iter: u32) -> Self {
        self.max_iter = max_iter;
        self
    }

    /// Test if a point `c = (cr, ci)` is in the Mandelbrot set.
    ///
    /// Returns `true` if the point didn't escape within `max_iter` iterations.
    pub fn is_in_set(&self, cr: f64, ci: f64) -> bool {
        let r = escape::escape_time(0.0, 0.0, cr, ci, self.max_iter, self.bailout_sq);
        !r.escaped
    }

    /// Compute escape-time iterations for point `c = (cr, ci)`.
    pub fn iterate(&self, cr: f64, ci: f64) -> EscapeResult {
        escape::escape_time(0.0, 0.0, cr, ci, self.max_iter, self.bailout_sq)
    }

    /// Render a row of the Mandelbrot set into iteration counts.
    ///
    /// Maps pixel coordinates `[0, width)` to complex values `[x_min, x_max]`
    /// at a fixed imaginary value `y`.
    pub fn render_row(&self, y: f64, x_min: f64, x_max: f64, width: usize) -> Vec<u32> {
        let mut row = Vec::with_capacity(width);
        let dx = (x_max - x_min) / width as f64;
        for i in 0..width {
            let cr = x_min + i as f64 * dx;
            let r = self.iterate(cr, y);
            row.push(r.iterations);
        }
        row
    }

    /// Render a full 2D grid of iteration counts.
    ///
    /// Returns `height` rows, each containing `width` iteration counts.
    pub fn render(&self, x_min: f64, x_max: f64, y_min: f64, y_max: f64, width: usize, height: usize) -> Vec<Vec<u32>> {
        let mut grid = Vec::with_capacity(height);
        let dy = (y_max - y_min) / height as f64;
        for j in 0..height {
            let y = y_min + j as f64 * dy;
            grid.push(self.render_row(y, x_min, x_max, width));
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mandelbrot_origin() {
        let m = Mandelbrot::new().with_max_iter(100);
        assert!(m.is_in_set(0.0, 0.0));
    }

    #[test]
    fn test_mandelbrot_outside() {
        let m = Mandelbrot::new().with_max_iter(100);
        assert!(!m.is_in_set(2.0, 0.0));
    }

    #[test]
    fn test_mandelbrot_known_inside() {
        let m = Mandelbrot::new().with_max_iter(500);
        // c = -0.5 + 0i is inside the main cardioid
        assert!(m.is_in_set(-0.5, 0.0));
    }

    #[test]
    fn test_mandelbrot_known_outside() {
        let m = Mandelbrot::new().with_max_iter(100);
        // c = 0.5 + 0i is outside
        assert!(!m.is_in_set(0.5, 0.0));
    }

    #[test]
    fn test_mandelbrot_iteration_count() {
        let m = Mandelbrot::new().with_max_iter(100);
        let r = m.iterate(2.0, 0.0);
        assert!(r.escaped);
        assert!(r.iterations < 5);
    }

    #[test]
    fn test_mandelbrot_render_row() {
        let m = Mandelbrot::new().with_max_iter(50);
        let row = m.render_row(0.0, -2.0, 2.0, 100);
        assert_eq!(row.len(), 100);
        // Origin should be max_iter (inside)
        assert_eq!(row[50], 50);
    }

    #[test]
    fn test_mandelbrot_render_grid() {
        let m = Mandelbrot::new().with_max_iter(50);
        let grid = m.render(-2.0, 2.0, -2.0, 2.0, 100, 100);
        assert_eq!(grid.len(), 100);
        assert_eq!(grid[0].len(), 100);
    }

    #[test]
    fn test_mandelbrot_symmetry() {
        let m = Mandelbrot::new().with_max_iter(100);
        let r1 = m.iterate(0.3, 0.5);
        let r2 = m.iterate(0.3, -0.5);
        assert_eq!(r1.iterations, r2.iterations);
    }

    #[test]
    fn test_mandelbrot_boundary() {
        let m = Mandelbrot::new().with_max_iter(1000);
        // c = -2.0 is exactly on the boundary of the main cardioid
        let r = m.iterate(-2.0, 0.0);
        // It should take many iterations or not escape
        assert!(r.iterations >= 100 || !r.escaped);
    }
}
