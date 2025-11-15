/// Module for TMA Engine: Geometry and Rendering
/// This crate provides tools for defining and manipulating 2D affine
/// transformations (TMAs) and rendering fractal structures generated
/// through Iterated Function Systems (IFS).
pub mod geometry;
pub mod render;

pub use geometry::{IFS, Point, TMA};
pub use render::Renderer;
