//! # fractal-gen-rs
//!
//! A pure-Rust fractal generation library providing Mandelbrot set, Julia set,
//! Burning Ship fractal, Sierpinski triangle, and Koch snowflake.
//!
//! # Example
//! ```
//! use fractal_gen_rs::mandelbrot::mandelbrot_iteration;
//! use fractal_gen_rs::julia::julia_iteration;
//! use fractal_gen_rs::burning_ship::burning_ship_iteration;
//!
//! // Mandelbrot: point (-0.5, 0.0) is in the set
//! assert_eq!(mandelbrot_iteration(-0.5, 0.0, 100), 100);
//!
//! // Julia set iteration
//! let iter = julia_iteration(0.0, 0.0, -0.7, 0.27015, 100);
//!
//! // Burning Ship iteration
//! let iter = burning_ship_iteration(-1.75, -0.02, 100);
//! ```

pub mod mandelbrot;
pub mod julia;
pub mod burning_ship;
pub mod sierpinski;
pub mod koch;
pub mod escape;
