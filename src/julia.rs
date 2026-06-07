//! Julia set computation.

use crate::escape::{self, EscapeResult};

/// Julia set evaluator for a fixed parameter `c`.
///
/// Each point `z₀` in the complex plane is iterated as `z = z² + c`.
/// Whether `z₀` belongs to the Julia set depends on whether it escapes.
pub struct Julia {
    /// Real part of the constant `c`.
    pub cr: f64,
    /// Imaginary part of the constant `c`.
    pub ci: f64,
    /// Maximum iterations.
    pub max_iter: u32,
    /// Squared bailout radius.
    pub bailout_sq: f64,
}

impl Julia {
    /// Create a Julia set evaluator for parameter `c = (cr, ci)`.
    pub fn new(cr: f64, ci: f64) -> Self {
        Self { cr, ci, max_iter: 256, bailout_sq: 4.0 }
    }

    /// Set maximum iterations.
    pub fn with_max_iter(mut self, max_iter: u32) -> Self {
        self.max_iter = max_iter;
        self
    }

    /// Test if starting point `z₀ = (zr, zi)` is in the filled Julia set.
    pub fn is_in_set(&self, zr: f64, zi: f64) -> bool {
        let r = escape::escape_time(zr, zi, self.cr, self.ci, self.max_iter, self.bailout_sq);
        !r.escaped
    }

    /// Compute escape-time iterations for starting point `z₀`.
    pub fn iterate(&self, zr: f64, zi: f64) -> EscapeResult {
        escape::escape_time(zr, zi, self.cr, self.ci, self.max_iter, self.bailout_sq)
    }

    /// Render a grid of iteration counts.
    pub fn render(&self, x_min: f64, x_max: f64, y_min: f64, y_max: f64, width: usize, height: usize) -> Vec<Vec<u32>> {
        let dx = (x_max - x_min) / width as f64;
        let dy = (y_max - y_min) / height as f64;
        let mut grid = Vec::with_capacity(height);
        for j in 0..height {
            let mut row = Vec::with_capacity(width);
            for i in 0..width {
                let zr = x_min + i as f64 * dx;
                let zi = y_min + j as f64 * dy;
                let r = self.iterate(zr, zi);
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
    fn test_julia_c_zero_contains_origin() {
        let j = Julia::new(0.0, 0.0).with_max_iter(100);
        // With c=0, z stays at its initial value; origin is in the set
        assert!(j.is_in_set(0.0, 0.0));
    }

    #[test]
    fn test_julia_c_zero_escape_far() {
        let j = Julia::new(0.0, 0.0).with_max_iter(100);
        // z0 = 3 should escape for c=0 (stays at 3, which is > bailout of 2)
        assert!(!j.is_in_set(3.0, 0.0));
    }

    #[test]
    fn test_julia_known_set() {
        // c = -0.7 + 0.27015i is a classic Julia set
        let j = Julia::new(-0.7, 0.27015).with_max_iter(200);
        // A point very close to origin should be in this Julia set
        // Actually, let's test a known point: z=0 with c=-0.123+0.745i (Douady rabbit)
        let j2 = Julia::new(-0.123, 0.745).with_max_iter(200);
        assert!(j2.is_in_set(0.0, 0.0));
    }

    #[test]
    fn test_julia_iteration_count() {
        let j = Julia::new(0.0, 0.0).with_max_iter(100);
        let r = j.iterate(3.0, 0.0);
        assert!(r.escaped);
        // z=3, z²=9, escapes immediately
        assert!(r.iterations <= 2);
    }

    #[test]
    fn test_julia_render() {
        let j = Julia::new(-0.7, 0.27015).with_max_iter(50);
        let grid = j.render(-2.0, 2.0, -2.0, 2.0, 50, 50);
        assert_eq!(grid.len(), 50);
        assert_eq!(grid[0].len(), 50);
    }

    #[test]
    fn test_julia_symmetry() {
        // With c = 0, z stays at z0, so z and -z have same behavior
        let j = Julia::new(0.0, 0.0).with_max_iter(100);
        let r1 = j.iterate(0.5, 0.3);
        let r2 = j.iterate(0.5, -0.3);
        assert_eq!(r1.iterations, r2.iterations);
    }

    #[test]
    fn test_julia_inside_small_disk() {
        // c = 0 + 0i: anything with |z| < 1 is in the set (z stays bounded)
        let j = Julia::new(0.0, 0.0).with_max_iter(200);
        assert!(j.is_in_set(0.5, 0.5));
        assert!(j.is_in_set(0.0, 0.9));
    }

    #[test]
    fn test_julia_outside_large() {
        let j = Julia::new(-0.7, 0.27015).with_max_iter(100);
        assert!(!j.is_in_set(10.0, 10.0));
    }
}
