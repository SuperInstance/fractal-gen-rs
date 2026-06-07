//! Escape-time algorithm: shared iteration logic for Mandelbrot, Julia, and Burning Ship.

/// Result of an escape-time computation.
#[derive(Clone, Copy, Debug)]
pub struct EscapeResult {
    /// Number of iterations before escape (or `max_iter` if it didn't escape).
    pub iterations: u32,
    /// Whether the point escaped (magnitude exceeded the bailout radius).
    pub escaped: bool,
    /// Final |z|² value (squared magnitude, avoids a sqrt).
    pub final_magnitude_sq: f64,
}

/// Run the standard escape-time iteration `z = z² + c` for complex values.
///
/// Starts at `zr, zi` and iterates with `cr, ci` until `|z|² > bailout_sq` or `max_iter` is reached.
pub fn escape_time(
    zr: f64,
    zi: f64,
    cr: f64,
    ci: f64,
    max_iter: u32,
    bailout_sq: f64,
) -> EscapeResult {
    let mut zr = zr;
    let mut zi = zi;
    let mut i = 0u32;
    let mut mag_sq = zr * zr + zi * zi;

    while i < max_iter && mag_sq <= bailout_sq {
        let new_zr = zr * zr - zi * zi + cr;
        let new_zi = 2.0 * zr * zi + ci;
        zr = new_zr;
        zi = new_zi;
        mag_sq = zr * zr + zi * zi;
        i += 1;
    }

    EscapeResult {
        iterations: i,
        escaped: mag_sq > bailout_sq,
        final_magnitude_sq: mag_sq,
    }
}

/// Run escape-time with the Burning Ship iteration: `z = (|Re(z)| + i|Im(z)|)² + c`.
pub fn escape_time_burning_ship(
    zr: f64,
    zi: f64,
    cr: f64,
    ci: f64,
    max_iter: u32,
    bailout_sq: f64,
) -> EscapeResult {
    let mut zr = zr;
    let mut zi = zi;
    let mut i = 0u32;
    let mut mag_sq = zr * zr + zi * zi;

    while i < max_iter && mag_sq <= bailout_sq {
        let ar = zr.abs();
        let ai = zi.abs();
        let new_zr = ar * ar - ai * ai + cr;
        let new_zi = 2.0 * ar * ai + ci;
        zr = new_zr;
        zi = new_zi;
        mag_sq = zr * zr + zi * zi;
        i += 1;
    }

    EscapeResult {
        iterations: i,
        escaped: mag_sq > bailout_sq,
        final_magnitude_sq: mag_sq,
    }
}

/// Compute smooth coloring value using normalized iteration count.
///
/// Returns a value in `[0.0, 1.0)` proportional to the iteration count,
/// smoothed for visual quality. Returns `0.0` for interior points.
pub fn smooth_color(result: EscapeResult, max_iter: u32) -> f64 {
    if !result.escaped {
        return 0.0;
    }
    // Normalized iteration count: n + 1 - log2(log2(|z|))
    let log_zn = 0.5 * result.final_magnitude_sq.ln();
    let nu = log_zn.ln() / (2.0_f64).ln();
    let smooth = (result.iterations as f64 + 1.0 - nu) / max_iter as f64;
    smooth.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_origin() {
        let r = escape_time(0.0, 0.0, 0.0, 0.0, 100, 4.0);
        assert!(!r.escaped);
        assert_eq!(r.iterations, 100);
    }

    #[test]
    fn test_escape_outside() {
        let r = escape_time(0.0, 0.0, 10.0, 0.0, 100, 4.0);
        assert!(r.escaped);
        assert!(r.iterations < 5);
    }

    #[test]
    fn test_escape_known_mandelbrot_inside() {
        // z=0, c=0 is in the Mandelbrot set
        let r = escape_time(0.0, 0.0, 0.0, 0.0, 1000, 4.0);
        assert!(!r.escaped);
    }

    #[test]
    fn test_escape_known_mandelbrot_outside() {
        // z=0, c=2+0i is outside the Mandelbrot set
        let r = escape_time(0.0, 0.0, 2.0, 0.0, 100, 4.0);
        assert!(r.escaped);
    }

    #[test]
    fn test_smooth_color_interior() {
        let r = EscapeResult { iterations: 100, escaped: false, final_magnitude_sq: 2.0 };
        assert_eq!(smooth_color(r, 100), 0.0);
    }

    #[test]
    fn test_smooth_color_exterior() {
        let r = EscapeResult { iterations: 50, escaped: true, final_magnitude_sq: 100.0 };
        let v = smooth_color(r, 100);
        assert!(v > 0.0 && v <= 1.0);
    }

    #[test]
    fn test_burning_ship_inside() {
        // Origin with c=0 should not escape
        let r = escape_time_burning_ship(0.0, 0.0, 0.0, 0.0, 100, 4.0);
        assert!(!r.escaped);
    }

    #[test]
    fn test_burning_ship_outside() {
        let r = escape_time_burning_ship(0.0, 0.0, 10.0, 10.0, 100, 4.0);
        assert!(r.escaped);
    }
}
