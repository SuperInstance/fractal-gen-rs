//! Koch snowflake generation.
//!
//! Generates the Koch snowflake fractal curve through recursive subdivision.
//! Starting from an equilateral triangle, each edge is divided into thirds
//! and a new equilateral triangle bump is added to the middle third.

/// Generate Koch snowflake vertices at a given recursion depth.
///
/// Returns the vertices of the closed Koch snowflake polygon.
/// At depth 0, returns an equilateral triangle.
/// Each additional depth multiplies the number of edges by 4.
///
/// # Arguments
/// * `depth` - Recursion depth (0 = triangle, 1 = 12 edges, etc.)
/// * `center` - Center of the snowflake as (x, y)
/// * `size` - Radius of the initial triangle
///
/// # Example
/// ```
/// use fractal_gen_rs::koch::koch_snowflake;
/// let vertices = koch_snowflake(2, (0.5, 0.5), 0.4);
/// assert!(vertices.len() > 3);
/// ```
pub fn koch_snowflake(depth: usize, center: (f64, f64), size: f64) -> Vec<(f64, f64)> {
    // Start with equilateral triangle
    let angle_offset = -std::f64::consts::PI / 2.0; // Point up
    let mut vertices: Vec<(f64, f64)> = (0..3)
        .map(|i| {
            let angle = angle_offset + 2.0 * std::f64::consts::PI * i as f64 / 3.0;
            (center.0 + size * angle.cos(), center.1 + size * angle.sin())
        })
        .collect();

    for _ in 0..depth {
        vertices = subdivide_koch(&vertices);
    }

    vertices
}

/// Perform one Koch subdivision step on a polygon.
fn subdivide_koch(vertices: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut new_vertices = Vec::with_capacity(vertices.len() * 4);
    let n = vertices.len();

    for i in 0..n {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % n];

        // Divide edge into thirds
        let ax = x1 + (x2 - x1) / 3.0;
        let ay = y1 + (y2 - y1) / 3.0;
        let bx = x1 + 2.0 * (x2 - x1) / 3.0;
        let by = y1 + 2.0 * (y2 - y1) / 3.0;

        // Peak of the equilateral triangle on the middle third
        // Rotate (bx-ax, by-ay) by -60 degrees and add to (ax, ay)
        let cos60 = 0.5;
        let sin60 = 3.0_f64.sqrt() / 2.0;
        let dx = bx - ax;
        let dy = by - ay;
        let px = ax + dx * cos60 - dy * sin60;
        let py = ay + dx * sin60 + dy * cos60;

        new_vertices.push((x1, y1));
        new_vertices.push((ax, ay));
        new_vertices.push((px, py));
        new_vertices.push((bx, by));
    }

    new_vertices
}

/// Compute the perimeter of the Koch snowflake at a given depth.
///
/// The initial triangle has perimeter 3 * side_length.
/// Each iteration multiplies the perimeter by 4/3.
pub fn koch_perimeter(depth: usize, side_length: f64) -> f64 {
    let initial_perimeter = 3.0 * side_length;
    initial_perimeter * (4.0_f64 / 3.0).powi(depth as i32)
}

/// Compute the number of edges at a given depth.
///
/// At depth 0: 3 edges, each depth multiplies by 4.
pub fn koch_edge_count(depth: usize) -> usize {
    3 * 4_usize.pow(depth as u32)
}

/// Compute the area of the Koch snowflake at a given depth.
///
/// The area converges to 8/5 of the original triangle area.
/// Area_n = Area_0 * (1 + sum of added triangle areas).
pub fn koch_area(depth: usize, side_length: f64) -> f64 {
    let triangle_area = (3.0_f64.sqrt() / 4.0) * side_length * side_length;
    let mut area = triangle_area;
    let mut added_area;
    let mut edge_count = 3;
    let mut current_side = side_length / 3.0;

    for _ in 0..depth {
        // Each edge gets a new triangle of side current_side
        added_area = (edge_count as f64) * (3.0_f64.sqrt() / 4.0) * current_side * current_side;
        area += added_area;
        edge_count *= 4;
        current_side /= 3.0;
    }

    area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth0_triangle() {
        let vertices = koch_snowflake(0, (0.5, 0.5), 0.4);
        assert_eq!(vertices.len(), 3);
    }

    #[test]
    fn test_depth1_edge_count() {
        let vertices = koch_snowflake(1, (0.5, 0.5), 0.4);
        assert_eq!(vertices.len(), 12);
    }

    #[test]
    fn test_depth2_edge_count() {
        let vertices = koch_snowflake(2, (0.5, 0.5), 0.4);
        assert_eq!(vertices.len(), 48);
    }

    #[test]
    fn test_koch_edge_count_function() {
        assert_eq!(koch_edge_count(0), 3);
        assert_eq!(koch_edge_count(1), 12);
        assert_eq!(koch_edge_count(2), 48);
        assert_eq!(koch_edge_count(3), 192);
    }

    #[test]
    fn test_koch_perimeter_growth() {
        let p0 = koch_perimeter(0, 1.0);
        let p1 = koch_perimeter(1, 1.0);
        assert!((p1 / p0 - 4.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_koch_perimeter_depth0() {
        let p = koch_perimeter(0, 1.0);
        assert!((p - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_koch_area_increases() {
        let a0 = koch_area(0, 1.0);
        let a1 = koch_area(1, 1.0);
        let a2 = koch_area(2, 1.0);
        assert!(a1 > a0);
        assert!(a2 > a1);
    }

    #[test]
    fn test_koch_area_convergence() {
        // Area should converge to 8/5 of original triangle
        let a0 = koch_area(0, 1.0);
        let a_limit = a0 * 8.0 / 5.0;
        let a10 = koch_area(10, 1.0);
        assert!((a10 - a_limit).abs() / a_limit < 0.01, "Area should be close to 8/5 limit");
    }

    #[test]
    fn test_snowflake_vertices_finite() {
        let vertices = koch_snowflake(5, (0.0, 0.0), 1.0);
        for (x, y) in &vertices {
            assert!(x.is_finite() && y.is_finite(), "Non-finite vertex");
        }
    }

    #[test]
    fn test_subdivide_preserves_start() {
        let tri = vec![(0.0, 0.0), (1.0, 0.0), (0.5, 0.866)];
        let subdivided = subdivide_koch(&tri);
        assert_eq!(subdivided[0], tri[0]);
    }

    #[test]
    fn test_koch_snowflake_center() {
        // Snowflake should be roughly centered around the given center
        let vertices = koch_snowflake(2, (0.5, 0.5), 0.4);
        let avg_x: f64 = vertices.iter().map(|v| v.0).sum::<f64>() / vertices.len() as f64;
        let avg_y: f64 = vertices.iter().map(|v| v.1).sum::<f64>() / vertices.len() as f64;
        assert!((avg_x - 0.5).abs() < 0.1, "Center x off: {}", avg_x);
        assert!((avg_y - 0.5).abs() < 0.1, "Center y off: {}", avg_y);
    }
}
