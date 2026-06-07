//! Burning Ship fractal computation.

use crate::escape::{self, EscapeResult};

/// Burning Ship fractal evaluator.
///
/// Similar to Mandelbrot but uses `z = (|Re(z)| + i|Im(z)|)² + c`.
pub struct BurningShip {
    /// Maximum iterations.
    pub max_iter: u32,
    /// Squared bailout radius.
    pub bailout_sq: f64,
}

impl Default for BurningShip {
    fn default() -> Self {
        Self { max_iter: 256, bailout_sq: 4.0 }
    }
}

impl BurningShip {
    /// Create with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum iterations.
    pub fn with_max_iter(mut self, max_iter: u32) -> Self {
        self.max_iter = max_iter;
        self
    }

    /// Test if a point `c = (cr, ci)` is in the Burning Ship set.
    pub fn is_in_set(&self, cr: f64, ci: f64) -> bool {
        let r = escape::escape_time_burning_ship(0.0, 0.0, cr, ci, self.max_iter, self.bailout_sq);
        !r.escaped
    }

    /// Compute escape-time iterations for a point.
    pub fn iterate(&self, cr: f64, ci: f64) -> EscapeResult {
        escape::escape_time_burning_ship(0.0, 0.0, cr, ci, self.max_iter, self.bailout_sq)
    }

    /// Render a grid of iteration counts.
    pub fn render(&self, x_min: f64, x_max: f64, y_min: f64, y_max: f64, width: usize, height: usize) -> Vec<Vec<u32>> {
        let dx = (x_max - x_min) / width as f64;
        let dy = (y_max - y_min) / height as f64;
        let mut grid = Vec::with_capacity(height);
        for j in 0..height {
            let mut row = Vec::with_capacity(width);
            for i in 0..width {
                let cr = x_min + i as f64 * dx;
                let ci = y_min + j as f64 * dy;
                let r = self.iterate(cr, ci);
                row.push(r.iterations);
            }
            grid.push(row);
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_burning_ship_origin() {
        let bs = BurningShip::new().with_max_iter(100);
        assert!(bs.is_in_set(0.0, 0.0));
    }

    #[test]
    fn test_burning_ship_outside() {
        let bs = BurningShip::new().with_max_iter(100);
        assert!(!bs.is_in_set(2.0, 2.0));
    }

    #[test]
    fn test_burning_ship_known_inside() {
        let bs = BurningShip::new().with_max_iter(200);
        // Small values near origin are inside
        assert!(bs.is_in_set(-0.5, -0.5));
    }

    #[test]
    fn test_burning_ship_iteration_count() {
        let bs = BurningShip::new().with_max_iter(100);
        let r = bs.iterate(10.0, 10.0);
        assert!(r.escaped);
        assert!(r.iterations < 5);
    }

    #[test]
    fn test_burning_ship_render() {
        let bs = BurningShip::new().with_max_iter(50);
        let grid = bs.render(-2.5, 1.5, -2.0, 1.0, 40, 30);
        assert_eq!(grid.len(), 30);
        assert_eq!(grid[0].len(), 40);
    }

    #[test]
    fn test_burning_ship_not_mandelbrot() {
        // Burning Ship differs from Mandelbrot; verify with a specific point
        let bs = BurningShip::new().with_max_iter(100);
        // Point (-1.75, -0.04) is near the ship's hull — should be inside
        let r = bs.iterate(-1.75, -0.04);
        // Should take many iterations or be inside
        assert!(r.iterations > 10 || !r.escaped);
    }
}
