//! Koch snowflake generator.

/// Generate a Koch snowflake as a list of line segments.
///
/// Starting from an equilateral triangle, each edge is subdivided `depth` times.
/// Returns vertices of the polygon approximation.
pub fn koch_snowflake(depth: u32) -> Vec<(f64, f64)> {
    // Start with equilateral triangle centered at origin
    let sqrt3_2 = 0.8660254037844386;
    let mut points = vec![
        (0.0, -1.0),
        (sqrt3_2, 0.5),
        (-sqrt3_2, 0.5),
    ];

    for _ in 0..depth {
        points = koch_subdivide(&points);
    }
    points
}

/// Subdivide each edge of a polygon by adding the Koch curve bump.
fn koch_subdivide(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let n = points.len();
    let mut new_points = Vec::with_capacity(n * 4);

    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];

        let dx = x2 - x1;
        let dy = y2 - y1;

        // Points at 1/3 and 2/3 of the edge
        let a = (x1 + dx / 3.0, y1 + dy / 3.0);
        let b = (x1 + 2.0 * dx / 3.0, y1 + 2.0 * dy / 3.0);

        // Peak of the equilateral triangle bump
        // Rotate vector (b - a) by 60 degrees and add to a
        let adx = b.0 - a.0;
        let ady = b.1 - a.1;
        let cos60 = 0.5;
        let sin60 = 0.8660254037844386;
        let peak = (a.0 + adx * cos60 - ady * sin60, a.1 + adx * sin60 + ady * cos60);

        new_points.push((x1, y1));
        new_points.push(a);
        new_points.push(peak);
        new_points.push(b);
    }

    new_points
}

/// Compute the perimeter of a Koch snowflake at the given depth.
///
/// The initial equilateral triangle has side length `side_length`.
/// Each subdivision multiplies the perimeter by 4/3.
pub fn koch_perimeter(side_length: f64, depth: u32) -> f64 {
    let initial = 3.0 * side_length;
    initial * (4.0_f64 / 3.0_f64).powi(depth as i32)
}

/// Compute the number of sides after `depth` subdivisions.
pub fn koch_side_count(depth: u32) -> usize {
    3 * 4usize.pow(depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_koch_depth0() {
        let pts = koch_snowflake(0);
        assert_eq!(pts.len(), 3); // Just the triangle
    }

    #[test]
    fn test_koch_depth1() {
        let pts = koch_snowflake(1);
        assert_eq!(pts.len(), 12); // 3 * 4
    }

    #[test]
    fn test_koch_depth2() {
        let pts = koch_snowflake(2);
        assert_eq!(pts.len(), 48); // 3 * 4^2
    }

    #[test]
    fn test_koch_depth3() {
        let pts = koch_snowflake(3);
        assert_eq!(pts.len(), 192); // 3 * 4^3
    }

    #[test]
    fn test_koch_perimeter_depth0() {
        let p = koch_perimeter(1.0, 0);
        assert!((p - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_koch_perimeter_depth1() {
        let p = koch_perimeter(1.0, 1);
        assert!((p - 4.0).abs() < 1e-10); // 3 * 4/3 = 4
    }

    #[test]
    fn test_koch_perimeter_grows() {
        let p0 = koch_perimeter(1.0, 0);
        let p1 = koch_perimeter(1.0, 1);
        let p2 = koch_perimeter(1.0, 2);
        assert!(p1 > p0);
        assert!(p2 > p1);
    }

    #[test]
    fn test_koch_side_count() {
        assert_eq!(koch_side_count(0), 3);
        assert_eq!(koch_side_count(1), 12);
        assert_eq!(koch_side_count(2), 48);
    }

    #[test]
    fn test_koch_points_bounded() {
        let pts = koch_snowflake(3);
        for (x, y) in &pts {
            assert!(x.abs() < 3.0 && y.abs() < 3.0, "point ({x},{y}) out of bounds");
        }
    }
}
