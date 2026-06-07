//! # fractal-gen-rs
//!
//! A pure-Rust library for fractal generation, providing:
//! - **Mandelbrot set** computation
//! - **Julia set** computation
//! - **Burning Ship** fractal
//! - **Sierpinski triangle** generator
//! - **Koch snowflake** generator
//! - **Escape-time** algorithm utilities

/// Mandelbrot set computation.
pub mod mandelbrot;
/// Julia set computation.
pub mod julia;
/// Burning Ship fractal computation.
pub mod burning_ship;
/// Sierpinski triangle generator.
pub mod sierpinski;
/// Koch snowflake generator.
pub mod koch;
/// Escape-time algorithm shared utilities.
pub mod escape;

pub use mandelbrot::Mandelbrot;
pub use julia::Julia;
pub use burning_ship::BurningShip;
