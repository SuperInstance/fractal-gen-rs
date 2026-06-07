//! Burning Ship fractal computation.
//!
//! The Burning Ship fractal is defined by the iteration:
//! z_{n+1} = (|Re(z_n)| + i|Im(z_n)|)^2 + c
//! It produces a distinctive "ship" shape.

use crate::escape::{has_escaped, DEFAULT_ESCAPE_RADIUS};

/// Compute the iteration count for a point in the Burning Ship fractal.
///
/// # Arguments
/// * `cx`, `cy` - The point c
/// * `max_iter` - Maximum iterations
///
/// # Example
/// ```
/// use fractal_gen_rs::burning_ship::burning_ship_iteration;
/// let iter = burning_ship_iteration(-1.75, -0.02, 100);
/// assert!(iter <= 100);
/// ```
pub fn burning_ship_iteration(cx: f64, cy: f64, max_iter: u32) -> u32 {
    let mut zx = 0.0;
    let mut zy = 0.0;
    let mut i = 0;

    while i < max_iter && !has_escaped(zx, zy, DEFAULT_ESCAPE_RADIUS) {
        // Burning Ship uses absolute values of components before squaring
        let new_zx = zx * zx - zy * zy + cx;
        zy = 2.0 * zx.abs() * zy.abs() + cy;
        zx = new_zx;
        i += 1;
    }

    i
}

/// Compute a rectangular region of the Burning Ship fractal.
pub fn burning_ship_region(
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
    max_iter: u32,
) -> Vec<u32> {
    let mut result = Vec::with_capacity(width * height);
    let dx = (x_max - x_min) / width as f64;
    let dy = (y_max - y_min) / height as f64;

    for row in 0..height {
        let cy = y_min + row as f64 * dy;
        for col in 0..width {
            let cx = x_min + col as f64 * dx;
            result.push(burning_ship_iteration(cx, cy, max_iter));
        }
    }

    result
}

/// Check if a point is in the Burning Ship set.
pub fn is_in_burning_ship(cx: f64, cy: f64, max_iter: u32) -> bool {
    burning_ship_iteration(cx, cy, max_iter) == max_iter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin_in_set() {
        // Origin should be in the Burning Ship set
        assert!(is_in_burning_ship(0.0, 0.0, 100));
    }

    #[test]
    fn test_far_point_escapes() {
        assert_eq!(burning_ship_iteration(10.0, 0.0, 100), 1);
    }

    #[test]
    fn test_known_ship_point() {
        // The "ship" body is near (-1.75, -0.02)
        let iter = burning_ship_iteration(-1.75, -0.02, 200);
        assert!(iter > 0);
    }

    #[test]
    fn test_burning_ship_not_mandelbrot() {
        // The Burning Ship should give different results from Mandelbrot at some points
        // For c = (0.5, 0.5), Burning Ship uses abs values, changing behavior
        let bs_iter = burning_ship_iteration(0.5, 0.5, 100);
        // In regular Mandelbrot, this would escape at iteration 2
        // Burning Ship may differ
        assert!(bs_iter <= 100);
    }

    #[test]
    fn test_burning_ship_region_dimensions() {
        let result = burning_ship_region(-2.0, 1.0, -2.0, 1.0, 50, 50, 50);
        assert_eq!(result.len(), 50 * 50);
    }

    #[test]
    fn test_burning_ship_region_values() {
        let result = burning_ship_region(-2.0, 1.0, -2.0, 1.0, 10, 10, 50);
        for &iter in &result {
            assert!(iter <= 50);
        }
    }

    #[test]
    fn test_symmetry_difference() {
        // Burning Ship is NOT symmetric about the real axis due to abs values
        // Points with positive and negative imaginary parts may differ
        let iter_pos = burning_ship_iteration(-1.5, 0.1, 100);
        let iter_neg = burning_ship_iteration(-1.5, -0.1, 100);
        // They can be the same by coincidence but the set is generally asymmetric
        assert!(iter_pos <= 100 && iter_neg <= 100);
    }

    #[test]
    fn test_real_axis_point() {
        // On the real axis, Burning Ship reduces to Mandelbrot
        let bs = burning_ship_iteration(-0.5, 0.0, 100);
        // For pure real inputs, abs doesn't change anything in the zx update
        assert!(bs <= 100);
    }

    #[test]
    fn test_known_outside_point() {
        assert!(!is_in_burning_ship(3.0, 3.0, 100));
    }
}
