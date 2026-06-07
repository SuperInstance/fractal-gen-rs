//! Sierpinski triangle generator.

/// A triangle defined by three 2D points.
pub type Triangle = ((f64, f64), (f64, f64), (f64, f64));

/// Generate Sierpinski triangle as a 2D boolean grid.
///
/// Uses the chaos game: start from any point inside the triangle,
/// repeatedly pick a random vertex and move halfway toward it.
/// After many iterations, the resulting points approximate the Sierpinski triangle.
pub fn sierpinski_chaos_game(iterations: usize, seed: u64) -> Vec<(f64, f64)> {
    let vertices = [(0.0, 0.0), (1.0, 0.0), (0.5, 0.8660254037844386)]; // equilateral triangle
    let mut points = Vec::with_capacity(iterations);
    let mut x = 0.5;
    let mut y = 0.3;
    let mut s = seed;

    for _ in 0..iterations {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let vi = (s >> 33) as usize % 3;
        x = (x + vertices[vi].0) / 2.0;
        y = (y + vertices[vi].1) / 2.0;
        points.push((x, y));
    }
    points
}

/// Generate Sierpinski triangle using recursive subdivision.
///
/// Returns triangles as `((x1,y1), (x2,y2), (x3,y3))` after `depth` levels of recursion.
pub fn sierpinski_subdivide(depth: u32) -> Vec<Triangle> {
    let a = (0.0, 0.0);
    let b = (1.0, 0.0);
    let c = (0.5, 0.8660254037844386);
    let mut triangles = Vec::new();
    subdivide(a, b, c, depth, &mut triangles);
    triangles
}

fn subdivide(
    a: (f64, f64),
    b: (f64, f64),
    c: (f64, f64),
    depth: u32,
    out: &mut Vec<Triangle>,
) {
    if depth == 0 {
        out.push((a, b, c));
        return;
    }
    let ab = mid(a, b);
    let bc = mid(b, c);
    let ca = mid(c, a);
    subdivide(a, ab, ca, depth - 1, out);
    subdivide(ab, b, bc, depth - 1, out);
    subdivide(ca, bc, c, depth - 1, out);
}

fn mid(p: (f64, f64), q: (f64, f64)) -> (f64, f64) {
    ((p.0 + q.0) / 2.0, (p.1 + q.1) / 2.0)
}

/// Test if a point `(x, y)` is inside the Sierpinski triangle using the bitwise method.
///
/// Maps coordinates to an integer grid and checks using the AND property:
/// a point is in the Sierpinski set iff `x & y == 0` in binary.
pub fn is_in_sierpinski(x: usize, y: usize) -> bool {
    x & y == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chaos_game_count() {
        let pts = sierpinski_chaos_game(1000, 42);
        assert_eq!(pts.len(), 1000);
    }

    #[test]
    fn test_chaos_game_bounds() {
        let pts = sierpinski_chaos_game(10000, 42);
        for (x, y) in &pts {
            assert!(*x >= 0.0 && *x <= 1.0, "x={x} out of bounds");
            assert!(*y >= 0.0 && *y <= 1.0, "y={y} out of bounds");
        }
    }

    #[test]
    fn test_chaos_game_deterministic() {
        let p1 = sierpinski_chaos_game(100, 42);
        let p2 = sierpinski_chaos_game(100, 42);
        for (a, b) in p1.iter().zip(p2.iter()) {
            assert!((a.0 - b.0).abs() < 1e-12);
            assert!((a.1 - b.1).abs() < 1e-12);
        }
    }

    #[test]
    fn test_subdivide_depth0() {
        let tris = sierpinski_subdivide(0);
        assert_eq!(tris.len(), 1); // Just the original triangle
    }

    #[test]
    fn test_subdivide_depth1() {
        let tris = sierpinski_subdivide(1);
        assert_eq!(tris.len(), 3); // 3 sub-triangles
    }

    #[test]
    fn test_subdivide_depth2() {
        let tris = sierpinski_subdivide(2);
        assert_eq!(tris.len(), 9); // 3^2
    }

    #[test]
    fn test_subdivide_depth3() {
        let tris = sierpinski_subdivide(3);
        assert_eq!(tris.len(), 27); // 3^3
    }

    #[test]
    fn test_sierpinski_bitwise_origin() {
        assert!(is_in_sierpinski(0, 0));
    }

    #[test]
    fn test_sierpinski_bitwise_diagonal() {
        // (1,1) -> 1 & 1 = 1 != 0, not in set
        assert!(!is_in_sierpinski(1, 1));
    }

    #[test]
    fn test_sierpinski_bitwise() {
        // (2,1) -> 10 & 01 = 0, in set
        assert!(is_in_sierpinski(2, 1));
        // (3,1) -> 11 & 01 = 1, not in set
        assert!(!is_in_sierpinski(3, 1));
    }
}
