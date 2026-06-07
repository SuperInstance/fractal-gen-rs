//! Escape-time algorithm utilities shared across fractal types.
//!
//! The escape-time algorithm iterates z = f(z, c) until |z| exceeds an escape radius
//! or a maximum iteration count is reached.

/// Default escape radius for most fractals.
pub const DEFAULT_ESCAPE_RADIUS: f64 = 2.0;

/// Compute the squared magnitude of a complex number (real, imag).
#[inline]
pub fn magnitude_squared(real: f64, imag: f64) -> f64 {
    real * real + imag * imag
}

/// Check if a point has escaped (|z| > escape_radius).
#[inline]
pub fn has_escaped(real: f64, imag: f64, escape_radius: f64) -> bool {
    magnitude_squared(real, imag) > escape_radius * escape_radius
}

/// Smooth iteration count for continuous coloring.
/// Uses the normalized iteration count algorithm.
///
/// Returns a smooth floating-point iteration count.
pub fn smooth_iteration(iteration: u32, max_iter: u32, real: f64, imag: f64) -> f64 {
    if iteration >= max_iter {
        max_iter as f64
    } else {
        let log_zn = (magnitude_squared(real, imag)).ln() / 2.0;
        let nu = (log_zn / 2.0_f64.ln()).ln() / 2.0_f64.ln();
        iteration as f64 + 1.0 - nu
    }
}

/// Map an iteration count to a grayscale value (0-255).
pub fn iteration_to_grayscale(iteration: u32, max_iter: u32) -> u8 {
    if iteration >= max_iter {
        0 // Inside the set = black
    } else {
        // Simple linear mapping
        ((iteration as f64 / max_iter as f64) * 255.0) as u8
    }
}

/// Map an iteration count to a color using a simple palette.
/// Returns (r, g, b) values in 0-255 range.
pub fn iteration_to_color(iteration: u32, max_iter: u32) -> (u8, u8, u8) {
    if iteration >= max_iter {
        (0, 0, 0)
    } else {
        let t = iteration as f64 / max_iter as f64;
        let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
        let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
        let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;
        (r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnitude_squared_zero() {
        assert!((magnitude_squared(0.0, 0.0) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_magnitude_squared_unit() {
        assert!((magnitude_squared(1.0, 0.0) - 1.0).abs() < 1e-10);
        assert!((magnitude_squared(0.0, 1.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_magnitude_squared_345() {
        // 3^2 + 4^2 = 25
        assert!((magnitude_squared(3.0, 4.0) - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_has_escaped_inside() {
        assert!(!has_escaped(0.5, 0.5, 2.0));
    }

    #[test]
    fn test_has_escaped_outside() {
        assert!(has_escaped(3.0, 0.0, 2.0));
    }

    #[test]
    fn test_has_escaped_on_boundary() {
        // |z| = 2.0 exactly, squared = 4.0, escape radius^2 = 4.0
        // should NOT escape (strict >)
        assert!(!has_escaped(2.0, 0.0, 2.0));
    }

    #[test]
    fn test_smooth_iteration_inside() {
        let val = smooth_iteration(100, 100, 0.0, 0.0);
        assert!((val - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_smooth_iteration_outside() {
        let val = smooth_iteration(10, 100, 3.0, 4.0);
        assert!(val > 0.0 && val < 100.0);
    }

    #[test]
    fn test_iteration_to_grayscale_inside() {
        assert_eq!(iteration_to_grayscale(100, 100), 0);
    }

    #[test]
    fn test_iteration_to_grayscale_escaped() {
        let val = iteration_to_grayscale(50, 100);
        assert!(val > 0);
    }

    #[test]
    fn test_iteration_to_color_inside() {
        assert_eq!(iteration_to_color(100, 100), (0, 0, 0));
    }

    #[test]
    fn test_iteration_to_color_escaped() {
        let (r, g, b) = iteration_to_color(50, 100);
        assert!(r > 0 || g > 0 || b > 0);
    }
}
