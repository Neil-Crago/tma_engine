# TMA Engine

[![Crates.io](https://img.shields.io/crates/v/tma_engine.svg)](https://crates.io/crates/tma_engine)
[![Docs.rs](https://docs.rs/tma_engine/badge.svg)](https://docs.rs/tma_engine)
[![CI](https://github.com/Neil-Crago/tma_engine/actions/workflows/rust.yml/badge.svg)](https://github.com/Neil-Crago/tma_engine/actions/workflows/rust.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

A modern Rust toolkit for composing affine transforms, building iterated function systems, and generating fractal point sets for rendering and experimentation.

`TMA` stands for Transformation, Matrix, Affine. The crate models a 2D affine map as:

$$T(v) = A v + c$$

This makes it straightforward to construct procedural fractals, simulate chaos-game iteration, and compose transformations of arbitrary complexity.

## Architectural approach

The library is intentionally structured as a small, compositional system with three distinct layers:

- `TMA` handles the local affine update rule itself.
- `IFS` handles how those local rules are selected and weighted over repeated iteration.
- `Renderer` is a separate projection layer for visualization, not the core of the mathematics.

This separation is important. It keeps the engine useful for topology-aware, branching, and generative systems where the point is not necessarily to render a pretty image, but to define a compact set of local rules that produce global structure under iteration. In other words, the crate is designed as a reusable generative substrate for self-similar systems rather than as a one-off fractal drawing tool.

## Features

- Clear 2D affine model with matrix and translation components.
- Operator overloads for composition and point application.
- Support for weighted IFS construction with automatic probability normalization.
- Convenience constructors for scaling, rotation, translation, and shear.
- Built-in image rendering for generated fractal point sets.
- Deterministic RNG injection for reproducible experiments and tests.

## Installation

```toml
[dependencies]
tma_engine = "0.2"
```

## Quick example

```rust
use tma_engine::{Point, TMA};

fn main() {
    let scale_half = TMA::from_scale(0.5);
    let rotate_90_deg = TMA::from_rotation(std::f64::consts::FRAC_PI_2);
    let translate_up = TMA::from_translation(0.0, 1.0);

    let composite = translate_up * rotate_90_deg * scale_half;
    let point: Point = [2.0, 0.0];

    let transformed = composite * point;
    println!("{:?}", transformed);
    assert_eq!(transformed, [0.0, 2.0]);
}
```

## Building an IFS

```rust
use tma_engine::geometry::{IFS, TMA};

fn main() {
    let triangle = vec![
        TMA::new([[0.5, 0.0], [0.0, 0.5]], [0.0, 0.0]).with_probability(1.0 / 3.0),
        TMA::new([[0.5, 0.0], [0.0, 0.5]], [0.5, 0.0]).with_probability(1.0 / 3.0),
        TMA::new([[0.5, 0.0], [0.0, 0.5]], [0.25, 0.5]).with_probability(1.0 / 3.0),
    ];

    let ifs = IFS::new(triangle).expect("valid IFS");
    let points = ifs.run_chaos_game(10_000, 100);
    println!("generated {} points", points.len());
}
```

## Example programs

The repository includes a few runnable examples, including visual and structural demonstrations:

```bash
cargo run --example sierpinski
cargo run --example barnsley_fern
cargo run --example affine_compose
cargo run --example vascular_branching
cargo run --example network_flow
```

The project also invites more general use cases, such as:

- recursive branching models
- vascular-like growth systems
- flow-network and routing patterns
- stochastic network generation
- topological template simulation using repeated affine updates
- non-image fractal analysis and sampling

## Research-oriented use cases

This crate is particularly well suited to forms of recursive geometry that are not best understood as images but as transformation rules on a structured space. Examples include:

- branching morphogenesis and vascular patterns
- recursive transport networks
- hierarchical routing or flow layouts
- procedural generation of self-similar topological templates
- stochastic systems where topology matters more than pixel output

The key idea is that repeated affine updates can act as a compact algebraic language for generating large-scale structure from small local rules. In this framing, the library is less about drawing fractals and more about describing the generative logic behind them.

## Research roadmap

A useful way to think about the architecture is in terms of problem classes:

- `TMA` as a local rule for growth, deformation, or transformation.
- `IFS` as a stochastic or weighted choice process over rules.
- `Renderer` as one optional view of the generated structure.

This translates well into real tasks such as:

- vascular or tree-like branching models
- recursive flow networks and routing heuristics
- topological templates for self-similar structures
- procedural generation of hierarchical systems
- coarse-grained simulation of network growth and adaptation

The crate is therefore best understood as a compact geometric substrate for recursive structure generation, with rendering as only one possible downstream interpretation.

## Topology metrics and graph semantics

The branch layer is intentionally graph-like. A `BranchNetwork` keeps parent pointers, child relationships, depth, and carrying-capacity metadata so a generated structure can be treated as a small topological object instead of only a rendering artifact.

Typical graph operations look like this:

```rust
use tma_engine::geometry::{BranchNetwork, IFS, TMA};

let ifs = IFS::new(vec![
    TMA::from_translation(1.0, 0.0).with_probability(0.6),
    TMA::from_translation(-1.0, 0.0).with_probability(0.4),
])
.expect("valid IFS");

let mut network = BranchNetwork::new([0.0, 0.0]);
network.grow_from_ifs(&ifs, &mut rand::thread_rng(), 2);

let root_children = network.children_of(0);
let visited = network.traverse_from(0);
let summary = network.flow_summary();

assert!(!root_children.is_empty());
assert!(visited.contains(&0));
assert!(summary.iter().all(|entry| entry.capacity >= entry.flow));
```

This makes the library useful for recursive branching, flow allocation, network morphology, and coarse geometric growth studies where the interesting object is the evolving topology rather than the final image.

## Non-visual fractal examples

The project is intentionally not limited to image generation. Two of the most useful examples are branch-like and flow-like structures that are meaningful as topological objects rather than rendered output.

- `vascular_branching` explores hierarchical recursive branching and physical spacing.
- `network_flow` models a weighted transport structure with aggregate flow and utilization.

These examples are valuable when the question is not "how does this look?" but rather "how does this grow, distribute, and sustain itself across repeated local updates?"

## Why it matters

This crate is intended for geometry-heavy tasks where transformation composition, probability-weighted iteration, and visual output all matter: fractal generation, procedural art, iterative geometry, and scientific visualization.

## Topology as a cousin

The affine model in `TMA` is not merely a visual trick for drawing pretty shapes. It also behaves like a local coordinate update rule, which is conceptually close to how topological systems are modeled: a map that acts on a local patch while preserving the structure needed for repeated gluing, recursion, and self-similarity.

In that sense, `TMA` sits near the boundary between:

- affine geometry
- topological templates
- iterative dynamical systems
- recursive branching models

A topological template can be thought of as a graph or atlas of local transformations. `TMA` gives a concrete, computable realization of those local rules. The difference is mostly one of emphasis: topology studies the invariants and connectivity, while `TMA` gives the practical algebra for composing, iterating, and evaluating those transforms in code.

## Non-visual fractals are often more useful

Purely visual fractals are compelling, but many of the most valuable fractal systems are not meant to be displayed as pictures at all.

Examples include:

- vascular and arterial branching networks
- tree-like transport systems
- recursive flow fields
- habitat or resource diffusion patterns
- local-to-global optimization structures

These are better understood as distributions, growth rules, branching maps, or weighted adjacency systems. In those contexts, the interesting question is not "how does it look?" but rather "how does it scale, branch, aggregate, and resist failure under repeated iteration?"

`TMA` is a useful substrate for exactly this class of problem: it offers a compact, robust language for describing repeated local transformations that can be used to simulate recursive branching, growth, and flow without committing to a specific visual representation.

## Author

Neil Crago

## License

This project is licensed under the MIT or Apache-2.0 licenses, at your option.