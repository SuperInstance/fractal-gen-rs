//! Mandelbrot set computation.
//!
//! The Mandelbrot set is defined as the set of complex numbers c for which
//! the iteration z_{n+1} = z_n^2 + c does not diverge when starting from z_0 = 0.

use crate::escape::{has_escaped, DEFAULT_ESCAPE_RADIUS};

/// Compute the iteration count for a point (cx, cy) in the Mandelbrot set.
///
/// Returns the number of iterations until escape, or `max_iter` if the point
/// is considered to be in the set.
///
/// # Arguments
/// * `cx` - Real part of the point c
/// * `cy` - Imaginary part of the point c
/// * `max_iter` - Maximum number of iterations before considering the point in the set
///
/// # Example
/// ```
/// use fractal_gen_rs::mandelbrot::mandelbrot_iteration;
/// // The origin is in the set
/// assert_eq!(mandelbrot_iteration(0.0, 0.0, 100), 100);
/// // A point far away escapes immediately
/// assert_eq!(mandelbrot_iteration(10.0, 0.0, 100), 1);
/// ```
pub fn mandelbrot_iteration(cx: f64, cy: f64, max_iter: u32) -> u32 {
    let mut zx = 0.0;
    let mut zy = 0.0;
    let mut i = 0;

    while i < max_iter && !has_escaped(zx, zy, DEFAULT_ESCAPE_RADIUS) {
        let new_zx = zx * zx - zy * zy + cx;
        zy = 2.0 * zx * zy + cy;
        zx = new_zx;
        i += 1;
    }

    i
}

/// Compute a rectangular region of the Mandelbrot set.
///
/// Returns a flat vector of iteration counts, row by row.
///
/// # Arguments
/// * `x_min`, `x_max` - Real axis bounds
/// * `y_min`, `y_max` - Imaginary axis bounds
/// * `width`, `height` - Resolution of the output
/// * `max_iter` - Maximum iterations per point
pub fn mandelbrot_region(
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
            result.push(mandelbrot_iteration(cx, cy, max_iter));
        }
    }

    result
}

/// Check if a point is likely in the Mandelbrot set (reaches max_iter without escaping).
pub fn is_in_mandelbrot(cx: f64, cy: f64, max_iter: u32) -> bool {
    mandelbrot_iteration(cx, cy, max_iter) == max_iter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin_in_set() {
        assert!(is_in_mandelbrot(0.0, 0.0, 100));
    }

    #[test]
    fn test_negative_one_quarter_in_set() {
        // c = -0.25 is in the Mandelbrot set (on the real axis, within the main cardioid)
        assert!(is_in_mandelbrot(-0.25, 0.0, 100));
    }

    #[test]
    fn test_minus_one_in_set() {
        // c = -1 is in the set (period-2 bulb)
        assert!(is_in_mandelbrot(-1.0, 0.0, 100));
    }

    #[test]
    fn test_point_outside_set() {
        // c = 2 + 0i is outside the set
        assert!(!is_in_mandelbrot(2.0, 0.0, 100));
    }

    #[test]
    fn test_far_point_escapes_quickly() {
        assert_eq!(mandelbrot_iteration(10.0, 0.0, 100), 1);
    }

    #[test]
    fn test_known_escape_count() {
        // c = 0.25 + 0.0i is just outside the set
        let iter = mandelbrot_iteration(0.251, 0.0, 1000);
        assert!(iter < 1000, "Should escape within 1000 iterations, got {}", iter);
    }

    #[test]
    fn test_main_cardioid_point() {
        // c = -0.75 + 0.5i is outside the main cardioid but may be in period-2 bulb
        // c = 1.0 + 0.5i is clearly outside the set
        assert!(!is_in_mandelbrot(1.0, 0.5, 100));
    }

    #[test]
    fn test_mandelbrot_region_dimensions() {
        let result = mandelbrot_region(-2.0, 1.0, -1.5, 1.5, 100, 100, 50);
        assert_eq!(result.len(), 100 * 100);
    }

    #[test]
    fn test_mandelbrot_region_values_valid() {
        let result = mandelbrot_region(-2.0, 1.0, -1.5, 1.5, 10, 10, 50);
        for &iter in &result {
            assert!(iter <= 50);
        }
    }

    #[test]
    fn test_boundary_detection() {
        // Points on the boundary should take many iterations
        // The tip of the main antenna is at c = -2.0
        let iter = mandelbrot_iteration(-2.0, 0.0, 1000);
        assert!(iter >= 1);
    }

    #[test]
    fn test_iteration_symmetry() {
        // The Mandelbrot set is symmetric about the real axis
        let iter_up = mandelbrot_iteration(-0.5, 0.5, 100);
        let iter_down = mandelbrot_iteration(-0.5, -0.5, 100);
        assert_eq!(iter_up, iter_down);
    }

    #[test]
    fn test_main_cardioid_interior() {
        // c = -0.1 + 0.1i is well inside the main cardioid
        assert!(is_in_mandelbrot(-0.1, 0.1, 200));
    }
}
