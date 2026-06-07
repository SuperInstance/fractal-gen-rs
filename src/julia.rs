//! Julia set computation.
//!
//! For a given constant c, the Julia set is the set of starting points z_0
//! for which the iteration z_{n+1} = z_n^2 + c remains bounded.

use crate::escape::{has_escaped, DEFAULT_ESCAPE_RADIUS};

/// Compute the iteration count for a point (zx, zy) in the Julia set with parameter c = (cx, cy).
///
/// # Arguments
/// * `zx` - Real part of starting point z
/// * `zy` - Imaginary part of starting point z
/// * `cx` - Real part of constant c
/// * `cy` - Imaginary part of constant c
/// * `max_iter` - Maximum iterations
///
/// # Example
/// ```
/// use fractal_gen_rs::julia::julia_iteration;
/// let iter = julia_iteration(0.0, 0.0, -0.7, 0.27015, 100);
/// assert!(iter <= 100);
/// ```
pub fn julia_iteration(zx: f64, zy: f64, cx: f64, cy: f64, max_iter: u32) -> u32 {
    let mut zx = zx;
    let mut zy = zy;
    let mut i = 0;

    while i < max_iter && !has_escaped(zx, zy, DEFAULT_ESCAPE_RADIUS) {
        let new_zx = zx * zx - zy * zy + cx;
        zy = 2.0 * zx * zy + cy;
        zx = new_zx;
        i += 1;
    }

    i
}

/// Compute a rectangular region of a Julia set.
///
/// Returns iteration counts in a flat vector.
    #[allow(clippy::too_many_arguments)]
pub fn julia_region(
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
    cx: f64,
    cy: f64,
    max_iter: u32,
) -> Vec<u32> {
    let mut result = Vec::with_capacity(width * height);
    let dx = (x_max - x_min) / width as f64;
    let dy = (y_max - y_min) / height as f64;

    for row in 0..height {
        let zy = y_min + row as f64 * dy;
        for col in 0..width {
            let zx = x_min + col as f64 * dx;
            result.push(julia_iteration(zx, zy, cx, cy, max_iter));
        }
    }

    result
}

/// Check if a point is in the Julia set (doesn't escape within max_iter).
pub fn is_in_julia(zx: f64, zy: f64, cx: f64, cy: f64, max_iter: u32) -> bool {
    julia_iteration(zx, zy, cx, cy, max_iter) == max_iter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin_julia_c0() {
        // With c = 0, z -> z^2, origin stays at 0
        assert!(is_in_julia(0.0, 0.0, 0.0, 0.0, 100));
    }

    #[test]
    fn test_far_point_escapes() {
        let iter = julia_iteration(10.0, 0.0, -0.7, 0.27015, 100);
        assert!(iter < 100, "Far point should escape, got {} iterations", iter);
    }

    #[test]
    fn test_julia_iteration_range() {
        for x in -5..=5 {
            for y in -5..=5 {
                let iter = julia_iteration(x as f64, y as f64, -0.7, 0.27015, 100);
                assert!(iter <= 100, "Iteration count exceeds max");
            }
        }
    }

    #[test]
    fn test_julia_c_negative_two() {
        // c = -2.0 is the tip point; the Julia set is a line segment
        // Points far from the real axis should escape
        assert!(!is_in_julia(0.0, 2.0, -2.0, 0.0, 100));
    }

    #[test]
    fn test_julia_region_dimensions() {
        let result = julia_region(-2.0, 2.0, -2.0, 2.0, 50, 50, -0.7, 0.27015, 100);
        assert_eq!(result.len(), 50 * 50);
    }

    #[test]
    fn test_julia_region_values_valid() {
        let result = julia_region(-2.0, 2.0, -2.0, 2.0, 10, 10, -0.7, 0.27015, 50);
        for &iter in &result {
            assert!(iter <= 50);
        }
    }

    #[test]
    fn test_julia_symmetry() {
        // Julia sets are symmetric about the origin for real c
        let cx = -1.0;
        let iter1 = julia_iteration(0.5, 0.5, cx, 0.0, 100);
        let iter2 = julia_iteration(-0.5, -0.5, cx, 0.0, 100);
        assert_eq!(iter1, iter2);
    }

    #[test]
    fn test_connected_julia_set() {
        // For c inside the Mandelbrot set, the Julia set is connected
        // c = 0 is trivially inside
        assert!(is_in_julia(0.0, 0.0, 0.0, 0.0, 100));
    }
}
