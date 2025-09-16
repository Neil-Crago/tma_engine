[![Crates.io](https://img.shields.io/crates/v/tma_engine.svg?style=flat-square)](https://crates.io/crates/tma_engine)
[![Docs.rs](https://img.shields.io/docsrs/tma_engine?style=flat-square)](https://docs.rs/tma_engine)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/Neil-Crago/tma_engine/actions/workflows/rust.yml/badge.svg)](https://github.com/Neil-Crago/tma_engine/actions/workflows/rust.yml)

# TMA Engine: Affine Transformation Toolkit

A lightweight and ergonomic Rust crate for defining, composing, and applying 2D affine transformations.

**TMA** stands for **T**ransformation, **M**atrix, **A**ffine. This engine provides the core algebraic structures for working with Iterated Function Systems (IFS), which are the mathematical foundation for generating a wide variety of fractals, including the Sierpinski Gasket and Barnsley's Fern.

## Features

- **Clear Algebraic Structure:** The `TMA` struct cleanly represents the transformation `T(v) = A*v + c`.
- **Operator Overloading:** Use the `*` operator to naturally compose transformations (`TMA * TMA`) or apply them to points (`TMA * Point`).
- **Stochastic IFS Support:** Includes an optional `probability` field in the `TMA` struct, essential for algorithms like the Chaos Game.
- **Helper Constructors:** Provides convenient methods for creating common transformations like scaling, rotation, and translation.

## Usage

Add this crate to your `Cargo.toml`:
```toml
[dependencies]
tma_engine = "0.1.8" # Or the latest version
```

## Example

```Rust
use tma_engine::{TMA, Point};

fn main() {
    let scale_half = TMA::from_scale(0.5);
    let rotate_90_deg = TMA::from_rotation(std::f64::consts::FRAC_PI_2);
    let translate_up = TMA::from_translation(0.0, 1.0);

    // Compose transformations using the multiplication operator.
    // The right-most operation is applied first.
    let composite_tma = translate_up * rotate_90_deg * scale_half;

    let my_point: Point = [2.0, 0.0];
    
    // Apply the composed transformation to a point.
    let new_point = composite_tma * my_point; 

    println!("Transformed point: {:?}", new_point);
    assert_eq!(new_point[0], 0.0);
    assert_eq!(new_point[1], 2.0);
}
```

## Purpose
This crate is a foundational component of the FractalAlgebra workspace. It provides the geometric building blocks for generating and exploring fractals and other systems based on affine transformations

## Author
Neil Crago — experimental mathematician

## Related Crates
This crate is part of a collection of crates by the same author:
These include:-
  * MOMA
  * MOMA_simulation_engine
  * fractal_algebra
  * factorial_engine
  * fa_slow_ai