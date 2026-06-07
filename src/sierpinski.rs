//! Sierpinski triangle generation.
//!
//! Generates points of the Sierpinski triangle using the chaos game algorithm
//! and deterministic recursive subdivision.

/// Generate Sierpinski triangle points using the chaos game algorithm.
///
/// Starting from a random point, repeatedly pick a random vertex and move
/// halfway toward it. After an initial warmup, all points lie on the fractal.
///
/// # Arguments
/// * `num_points` - Number of points to generate
/// * `seed` - Seed for the pseudo-random number generator
///
/// # Returns
/// Vector of (x, y) coordinates, each in [0, 1] x [0, 1].
pub fn sierpinski_chaos_game(num_points: usize, seed: u64) -> Vec<(f64, f64)> {
    // Equilateral triangle vertices
    let vertices = [
        (0.5, 0.0),                  // Top
        (0.0, 3.0_f64.sqrt() / 2.0), // Bottom-left
        (1.0, 3.0_f64.sqrt() / 2.0), // Bottom-right
    ];

    let mut points = Vec::with_capacity(num_points);
    let mut x = 0.5;
    let mut y = 0.5;
    let mut rng_state = seed;

    // Warmup (20 iterations to converge)
    for _ in 0..20 {
        rng_state = simple_xorshift(rng_state);
        let idx = (rng_state % 3) as usize;
        x = (x + vertices[idx].0) / 2.0;
        y = (y + vertices[idx].1) / 2.0;
    }

    for _ in 0..num_points {
        rng_state = simple_xorshift(rng_state);
        let idx = (rng_state % 3) as usize;
        x = (x + vertices[idx].0) / 2.0;
        y = (y + vertices[idx].1) / 2.0;
        points.push((x, y));
    }

    points
}

/// Simple xorshift64 pseudo-random number generator.
fn simple_xorshift(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
}

/// Check if a point (x, y) lies on the Sierpinski triangle (approximately).
///
/// Uses the property that in binary representation, the Sierpinski triangle
/// excludes points where both coordinates have a 1 bit at the same position.
pub fn is_on_sierpinski(x: f64, y: f64, depth: usize) -> bool {
    // Convert to integer grid at the given depth
    let grid_size = 1 << depth;
    let ix = (x * grid_size as f64) as usize;
    let iy = (y * grid_size as f64) as usize;

    if ix >= grid_size || iy >= grid_size {
        return false;
    }

    // Check: for each bit position, not both should be 1
    let mut bx = ix;
    let mut by = iy;
    for _ in 0..depth {
        if (bx & 1) == 1 && (by & 1) == 1 {
            return false;
        }
        bx >>= 1;
        by >>= 1;
    }
    true
}

/// A triangle defined by three 2D points.
pub type Triangle = ((f64, f64), (f64, f64), (f64, f64));

/// Generate the vertices of a Sierpinski triangle at a given recursion depth.
///
/// Returns the triangles as triplets of ((x1,y1), (x2,y2), (x3,y3)).
pub fn sierpinski_triangles(depth: usize) -> Vec<Triangle> {
    let v1 = (0.5, 0.0);
    let v2 = (0.0, 1.0);
    let v3 = (1.0, 1.0);
    let mut triangles = vec![(v1, v2, v3)];
    let mut next = Vec::new();

    for _ in 0..depth {
        next.clear();
        for &(a, b, c) in &triangles {
            let ab = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
            let bc = ((b.0 + c.0) / 2.0, (b.1 + c.1) / 2.0);
            let ac = ((a.0 + c.0) / 2.0, (a.1 + c.1) / 2.0);
            next.push((a, ab, ac));
            next.push((ab, b, bc));
            next.push((ac, bc, c));
        }
        triangles = next.clone();
    }

    triangles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chaos_game_point_count() {
        let points = sierpinski_chaos_game(100, 42);
        assert_eq!(points.len(), 100);
    }

    #[test]
    fn test_chaos_game_points_in_bounds() {
        let points = sierpinski_chaos_game(1000, 42);
        for (x, y) in &points {
            assert!(*x >= 0.0 && *x <= 1.0, "x={x} out of bounds");
            assert!(*y >= 0.0 && *y <= 1.0, "y={y} out of bounds");
        }
    }

    #[test]
    fn test_chaos_game_deterministic() {
        let p1 = sierpinski_chaos_game(50, 42);
        let p2 = sierpinski_chaos_game(50, 42);
        for (a, b) in p1.iter().zip(p2.iter()) {
            assert!((a.0 - b.0).abs() < 1e-10);
            assert!((a.1 - b.1).abs() < 1e-10);
        }
    }

    #[test]
    fn test_is_on_sierpinski_corner() {
        // Corner points should be on the triangle
        assert!(is_on_sierpinski(0.0, 0.0, 5));
    }

    #[test]
    fn test_is_on_sierpinski_center_excluded() {
        // The exact center (0.5, 0.5) at depth 1 should be excluded
        // At depth 1: ix=1, iy=1 -> both have bit 0 set -> excluded
        assert!(!is_on_sierpinski(0.51, 0.51, 1));
    }

    #[test]
    fn test_is_on_sierpinski_valid_point() {
        // (0.25, 0.25) at depth 2: ix=1, iy=1 -> bit 0 set for both -> excluded
        assert!(!is_on_sierpinski(0.26, 0.26, 2));
    }

    #[test]
    fn test_sierpinski_triangles_depth0() {
        let triangles = sierpinski_triangles(0);
        assert_eq!(triangles.len(), 1);
    }

    #[test]
    fn test_sierpinski_triangles_depth1() {
        let triangles = sierpinski_triangles(1);
        assert_eq!(triangles.len(), 3);
    }

    #[test]
    fn test_sierpinski_triangles_depth_n() {
        // Each level triples the number of triangles
        let t2 = sierpinski_triangles(2);
        assert_eq!(t2.len(), 9);
        let t3 = sierpinski_triangles(3);
        assert_eq!(t3.len(), 27);
    }

    #[test]
    fn test_xorshift_nonzero() {
        let mut state = 42u64;
        for _ in 0..100 {
            state = simple_xorshift(state);
            assert_ne!(state, 0, "xorshift produced zero");
        }
    }

    #[test]
    fn test_chaos_game_different_seeds() {
        let p1 = sierpinski_chaos_game(10, 1);
        let p2 = sierpinski_chaos_game(10, 2);
        let mut any_different = false;
        for (a, b) in p1.iter().zip(p2.iter()) {
            if (a.0 - b.0).abs() > 1e-10 || (a.1 - b.1).abs() > 1e-10 {
                any_different = true;
                break;
            }
        }
        assert!(any_different, "Different seeds produced identical output");
    }
}
