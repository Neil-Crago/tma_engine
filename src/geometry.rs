/// Module for 2D affine transformations and Iterated Function Systems (IFS).
/// This module defines the `TMA` type for affine transformations and the `IFS`
/// type for managing collections of transformations that generate fractal
/// structures through stochastic iteration.
use std::fmt;
use std::ops::Mul;

use rand::Rng;

/// A point in 2D affine space.
pub type Point = [f64; 2];

/// The error produced when constructing an `IFS` from transformations that do
/// not define a valid stochastic system.
#[derive(Debug, Clone, PartialEq)]
pub enum IFSBuildError {
    /// The collection of transformations is empty.
    Empty,
    /// At least one transformation does not declare a probability.
    MissingProbability,
    /// The total probability is zero or not finite.
    InvalidProbabilities,
}

impl fmt::Display for IFSBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "IFS cannot be empty."),
            Self::MissingProbability => {
                write!(
                    f,
                    "All TMAs in an IFS must have a probability for stochastic generation."
                )
            }
            Self::InvalidProbabilities => {
                write!(
                    f,
                    "Probability values must be finite and sum to a positive value."
                )
            }
        }
    }
}

impl std::error::Error for IFSBuildError {}

/// TMA: Transformation, Matrix, Affine.
/// Represents a 2D affine transformation `T(v) = A * v + c`.
/// This is a fundamental building block for Iterated Function Systems (IFS).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TMA {
    /// The 2x2 matrix `A` for linear transformations, such as scaling,
    /// rotation, reflection, and shear.
    pub matrix: [[f64; 2]; 2],

    /// The 2D translation vector `c`.
    pub vector: Point,

    /// Optional probability for use in stochastic IFS generation.
    /// The weights are normalized internally when constructing an `IFS`.
    pub probability: Option<f64>,
}

impl TMA {
    /// Creates a new affine transform from a given matrix and translation vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tma_engine::TMA;
    ///
    /// let transform = TMA::new([[2.0, 0.0], [0.0, 2.0]], [1.0, -1.0]);
    /// let point = transform * [2.0, 3.0];
    ///
    /// assert_eq!(point, [5.0, 5.0]);
    /// ```
    pub fn new(matrix: [[f64; 2]; 2], vector: Point) -> Self {
        TMA {
            matrix,
            vector,
            probability: None,
        }
    }

    /// Creates an identity transformation.
    pub fn identity() -> Self {
        TMA {
            matrix: [[1.0, 0.0], [0.0, 1.0]],
            vector: [0.0, 0.0],
            probability: None,
        }
    }

    /// Creates a uniform scale transform.
    pub fn from_scale(s: f64) -> Self {
        TMA::new([[s, 0.0], [0.0, s]], [0.0, 0.0])
    }

    /// Creates a translation transform.
    ///
    /// ```rust
    /// use tma_engine::TMA;
    ///
    /// let transform = TMA::from_translation(3.0, 4.0);
    /// let point = transform * [1.0, 2.0];
    ///
    /// assert_eq!(point, [4.0, 6.0]);
    /// ```
    pub fn from_translation(tx: f64, ty: f64) -> Self {
        TMA::new([[1.0, 0.0], [0.0, 1.0]], [tx, ty])
    }

    /// Creates a rotation transform using radians.
    pub fn from_rotation(theta: f64) -> Self {
        let (sin_t, cos_t) = theta.sin_cos();
        TMA::new([[cos_t, -sin_t], [sin_t, cos_t]], [0.0, 0.0])
    }

    /// Creates a shear transform.
    pub fn from_shear(xy: f64, yx: f64) -> Self {
        TMA::new([[1.0, xy], [yx, 1.0]], [0.0, 0.0])
    }

    /// Attaches a non-negative weight to the transformation. The `IFS` builder
    /// normalizes all weights so they sum to 1.0 during stochastic selection.
    ///
    /// ```rust
    /// use tma_engine::TMA;
    ///
    /// let t = TMA::from_translation(1.0, 0.0).with_probability(0.7);
    /// assert_eq!(t.probability, Some(0.7));
    /// ```
    pub fn with_probability(mut self, p: f64) -> Self {
        self.probability = Some(p);
        self
    }

    /// Applies the transformation to a point.
    pub fn apply(&self, p: Point) -> Point {
        let x = p[0];
        let y = p[1];

        let new_x = self.matrix[0][0] * x + self.matrix[0][1] * y + self.vector[0];
        let new_y = self.matrix[1][0] * x + self.matrix[1][1] * y + self.vector[1];

        [new_x, new_y]
    }

    /// Composes this transformation with another one.
    /// This is equivalent to `self * other`, which applies `other` first and
    /// then `self`.
    ///
    /// ```rust
    /// use tma_engine::TMA;
    ///
    /// let scale = TMA::from_scale(2.0);
    /// let translate = TMA::from_translation(3.0, 4.0);
    /// let composed = translate.compose(&scale);
    ///
    /// assert_eq!(composed * [1.0, 2.0], [5.0, 8.0]);
    /// ```
    pub fn compose(&self, other: &TMA) -> Self {
        let m1 = self.matrix;
        let m2 = other.matrix;
        let new_matrix = [
            [
                m1[0][0] * m2[0][0] + m1[0][1] * m2[1][0],
                m1[0][0] * m2[0][1] + m1[0][1] * m2[1][1],
            ],
            [
                m1[1][0] * m2[0][0] + m1[1][1] * m2[1][0],
                m1[1][0] * m2[0][1] + m1[1][1] * m2[1][1],
            ],
        ];

        let c1 = self.vector;
        let c2 = other.vector;
        let new_vector = [
            m1[0][0] * c2[0] + m1[0][1] * c2[1] + c1[0],
            m1[1][0] * c2[0] + m1[1][1] * c2[1] + c1[1],
        ];

        TMA::new(new_matrix, new_vector)
    }
}

impl Mul<TMA> for TMA {
    type Output = TMA;

    fn mul(self, rhs: TMA) -> Self::Output {
        self.compose(&rhs)
    }
}

impl Mul<Point> for TMA {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        self.apply(rhs)
    }
}

/// A collection of mutually weighted affine transforms used to generate an IFS.
#[derive(Debug, PartialEq)]
pub struct IFS {
    transformations: Vec<TMA>,
    cumulative_probs: Vec<f64>,
}

impl IFS {
    /// Returns the underlying transformation list.
    pub fn transformations(&self) -> &[TMA] {
        &self.transformations
    }

    /// Creates an IFS from a vector of transformations.
    ///
    /// Individual probabilities are normalized automatically so that the total
    /// probability mass is 1.0, making the construction more forgiving than the
    /// original strict "must sum to exactly 1.0" model.
    ///
    /// ```rust
    /// use tma_engine::geometry::{IFS, TMA};
    ///
    /// let ifs = IFS::new(vec![
    ///     TMA::new([[0.5, 0.0], [0.0, 0.5]], [0.0, 0.0]).with_probability(0.5),
    ///     TMA::new([[0.5, 0.0], [0.0, 0.5]], [0.5, 0.0]).with_probability(0.5),
    /// ])
    /// .expect("valid IFS");
    ///
    /// let points = ifs.run_chaos_game(10, 0);
    /// assert_eq!(points.len(), 10);
    /// ```
    pub fn new(transformations: Vec<TMA>) -> Result<Self, IFSBuildError> {
        if transformations.is_empty() {
            return Err(IFSBuildError::Empty);
        }

        let mut probs = Vec::with_capacity(transformations.len());
        let mut total_prob = 0.0;

        for tma in &transformations {
            let probability = tma.probability.ok_or(IFSBuildError::MissingProbability)?;
            if !probability.is_finite() || probability < 0.0 {
                return Err(IFSBuildError::InvalidProbabilities);
            }
            total_prob += probability;
            probs.push(probability);
        }

        if !total_prob.is_finite() || total_prob <= 0.0 {
            return Err(IFSBuildError::InvalidProbabilities);
        }

        let mut cumulative_probs = Vec::with_capacity(transformations.len());
        let mut running_total = 0.0;

        for probability in probs {
            running_total += probability / total_prob;
            cumulative_probs.push(running_total);
        }

        Ok(IFS {
            transformations,
            cumulative_probs,
        })
    }

    /// Returns the normalized cumulative probabilities for the active set.
    pub fn cumulative_probabilities(&self) -> &[f64] {
        &self.cumulative_probs
    }

    /// Chooses a transformation index using a supplied RNG.
    pub fn choose_index<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        let r = rng.gen_range(0.0..1.0);

        self.cumulative_probs
            .iter()
            .position(|&cumulative_prob| r < cumulative_prob)
            .unwrap_or_else(|| self.transformations.len().saturating_sub(1))
    }

    /// Chooses a transformation using a supplied RNG.
    pub fn choose_transformation<R: Rng + ?Sized>(&self, rng: &mut R) -> &TMA {
        let index = self.choose_index(rng);
        &self.transformations[index]
    }

    /// Chooses a transformation using the thread-local RNG.
    pub fn choose_transformation_thread_rng(&self) -> &TMA {
        let mut rng = rand::thread_rng();
        self.choose_transformation(&mut rng)
    }

    /// Runs the Chaos Game using the thread-local RNG.
    pub fn run_chaos_game(
        &self,
        num_points: usize,
        warmup_iterations: usize,
    ) -> Vec<(Point, usize)> {
        let mut rng = rand::thread_rng();
        self.run_chaos_game_with_rng(num_points, warmup_iterations, &mut rng)
    }

    /// Runs the Chaos Game using a caller-provided RNG.
    pub fn run_chaos_game_with_rng<R: Rng + ?Sized>(
        &self,
        num_points: usize,
        warmup_iterations: usize,
        rng: &mut R,
    ) -> Vec<(Point, usize)> {
        let mut points = Vec::with_capacity(num_points);
        let mut current_point: Point = [0.0, 0.0];
        let total_iterations = num_points + warmup_iterations;

        for iteration in 0..total_iterations {
            let chosen_index = self.choose_index(rng);
            let tma = &self.transformations[chosen_index];
            current_point = tma.apply(current_point);

            if iteration >= warmup_iterations {
                points.push((current_point, chosen_index));
            }
        }

        points
    }
}

/// A node in a topological or branching structure generated from affine rules.
#[derive(Debug, Clone, PartialEq)]
pub struct BranchNode {
    /// The geometric location of the branch node.
    pub point: Point,
    /// The branching depth of this node.
    pub depth: usize,
    /// The accumulated flow or weight at the node.
    pub flow: f64,
    /// The effective carrying capacity of the branch beyond the current flow.
    pub capacity: f64,
    /// The parent index, if this node is not the root.
    pub parent: Option<usize>,
}

/// A directed edge in the recursive branching network.
#[derive(Debug, Clone, PartialEq)]
pub struct BranchEdge {
    pub from: usize,
    pub to: usize,
    pub weight: f64,
    pub capacity: f64,
}

/// A summary of the flow state at a branch node.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlowSummaryEntry {
    pub node_index: usize,
    pub depth: usize,
    pub flow: f64,
    pub capacity: f64,
    pub utilization: f64,
}

/// A recursive branching structure built from an `IFS`.
///
/// This type is also exposed as a graph-like object for explicit parent/child
/// traversal and topological metrics.
#[derive(Debug, Clone, PartialEq)]
pub struct BranchNetwork {
    nodes: Vec<BranchNode>,
    edges: Vec<BranchEdge>,
}

/// Alias for the branch network as a graph-oriented structure.
pub type BranchGraph = BranchNetwork;

impl BranchNetwork {
    /// Creates a branch network with a single root node.
    pub fn new(root_point: Point) -> Self {
        Self {
            nodes: vec![BranchNode {
                point: root_point,
                depth: 0,
                flow: 1.0,
                capacity: 1.0,
                parent: None,
            }],
            edges: Vec::new(),
        }
    }

    /// Returns the root node.
    pub fn root(&self) -> &BranchNode {
        &self.nodes[0]
    }

    /// Returns the root index.
    pub fn root_index(&self) -> usize {
        0
    }

    /// Returns the node list.
    pub fn nodes(&self) -> &[BranchNode] {
        &self.nodes
    }

    /// Returns the edge list.
    pub fn edges(&self) -> &[BranchEdge] {
        &self.edges
    }

    /// Returns the number of nodes currently in the network.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns true when the network contains only the root node.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns the depth of the deepest node in the network.
    pub fn max_depth(&self) -> usize {
        self.nodes.iter().map(|node| node.depth).max().unwrap_or(0)
    }

    /// Returns the total flow currently in the network.
    pub fn total_flow(&self) -> f64 {
        self.nodes.iter().map(|node| node.flow).sum()
    }

    /// Returns the aggregate carrying capacity across the network.
    pub fn total_capacity(&self) -> f64 {
        self.nodes.iter().map(|node| node.capacity).sum()
    }

    /// Returns a compact flow summary for each node in the network.
    pub fn flow_summary(&self) -> Vec<FlowSummaryEntry> {
        self.nodes
            .iter()
            .enumerate()
            .map(|(index, node)| {
                let utilization = if node.capacity > 0.0 {
                    node.flow / node.capacity
                } else {
                    0.0
                };

                FlowSummaryEntry {
                    node_index: index,
                    depth: node.depth,
                    flow: node.flow,
                    capacity: node.capacity,
                    utilization,
                }
            })
            .collect()
    }

    /// Returns the direct parent of a node, if it exists.
    pub fn parent_of(&self, node_index: usize) -> Option<usize> {
        self.nodes.get(node_index).and_then(|node| node.parent)
    }

    /// Returns the stored depth of a node, if it exists.
    pub fn node_depth(&self, node_index: usize) -> Option<usize> {
        self.nodes.get(node_index).map(|node| node.depth)
    }

    /// Returns the capacity recorded on a node, if it exists.
    pub fn node_capacity(&self, node_index: usize) -> Option<f64> {
        self.nodes.get(node_index).map(|node| node.capacity)
    }

    /// Returns the children of a node.
    pub fn children_of(&self, node_index: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|edge| (edge.from == node_index).then_some(edge.to))
            .collect()
    }

    /// Traverses the directed branch graph from a node and returns a parent/child
    /// ordering in breadth-first sequence.
    pub fn traverse_from(&self, start_index: usize) -> Vec<usize> {
        if self.nodes.get(start_index).is_none() {
            return Vec::new();
        }

        let mut order = Vec::new();
        let mut visited = vec![false; self.nodes.len()];
        let mut frontier = std::collections::VecDeque::from([start_index]);
        visited[start_index] = true;

        while let Some(index) = frontier.pop_front() {
            order.push(index);
            for child in self.children_of(index) {
                if !visited[child] {
                    visited[child] = true;
                    frontier.push_back(child);
                }
            }
        }

        order
    }

    /// Grows the network by repeatedly applying an `IFS` to each active node.
    pub fn grow_from_ifs<R: Rng + ?Sized>(&mut self, ifs: &IFS, rng: &mut R, depth_limit: usize) {
        let mut frontier = std::collections::VecDeque::from([0usize]);

        while let Some(index) = frontier.pop_front() {
            let current = self.nodes[index].clone();
            if current.depth >= depth_limit {
                continue;
            }

            for _ in 0..2 {
                let chosen = ifs.choose_index(rng);
                let transform = &ifs.transformations()[chosen];
                let next_point = transform.apply(current.point);
                let next_index = self.nodes.len();
                let next_flow = current.flow * 0.9;
                let next_capacity = current.capacity * 0.9 + current.flow * 0.1;

                self.nodes.push(BranchNode {
                    point: next_point,
                    depth: current.depth + 1,
                    flow: next_flow,
                    capacity: next_capacity,
                    parent: Some(index),
                });
                self.edges.push(BranchEdge {
                    from: index,
                    to: next_index,
                    weight: current.flow,
                    capacity: next_capacity,
                });
                frontier.push_back(next_index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_probability_preserves_weight_for_normalization() {
        let weighted = TMA::identity().with_probability(2.0);
        let negative = TMA::identity().with_probability(-0.1);

        assert_eq!(weighted.probability, Some(2.0));
        assert_eq!(negative.probability, Some(-0.1));
    }

    #[test]
    fn composition_applies_transforms_in_order() {
        let scale = TMA::from_scale(2.0);
        let translate = TMA::from_translation(3.0, 4.0);

        let composed = translate * scale;
        let out = composed.apply([1.0, 2.0]);

        assert_eq!(out, [5.0, 8.0]);
    }

    #[test]
    fn ifs_normalizes_probabilities() {
        let ifs = IFS::new(vec![
            TMA::identity().with_probability(2.0),
            TMA::identity().with_probability(1.0),
        ])
        .expect("valid IFS should be created");

        assert_eq!(ifs.cumulative_probabilities().len(), 2);
        assert!((ifs.cumulative_probabilities()[0] - 0.666_666_666_666_666_6).abs() < 1e-12);
        assert!((ifs.cumulative_probabilities()[1] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn ifs_rejects_negative_probabilities() {
        let result = IFS::new(vec![
            TMA::identity().with_probability(0.5),
            TMA::identity().with_probability(-0.1),
        ]);

        assert_eq!(result, Err(IFSBuildError::InvalidProbabilities));
    }

    #[test]
    fn branch_network_tracks_recursive_growth() {
        let ifs = IFS::new(vec![
            TMA::from_translation(1.0, 0.0).with_probability(0.6),
            TMA::from_translation(-1.0, 0.0).with_probability(0.4),
        ])
        .expect("valid flow network");

        let mut network = BranchNetwork::new([0.0, 0.0]);
        network.grow_from_ifs(&ifs, &mut rand::thread_rng(), 2);

        assert!(network.len() > 1);
        assert!(network.root().depth == 0);
    }

    #[test]
    fn branch_network_tracks_edges_between_parents_and_children() {
        let ifs = IFS::new(vec![
            TMA::from_translation(1.0, 0.0).with_probability(0.6),
            TMA::from_translation(-1.0, 0.0).with_probability(0.4),
        ])
        .expect("valid flow network");

        let mut network = BranchNetwork::new([0.0, 0.0]);
        network.grow_from_ifs(&ifs, &mut rand::thread_rng(), 1);

        assert!(!network.edges().is_empty());
        assert!(network.edges()[0].to > network.edges()[0].from);
        assert!(network.nodes()[0].depth == 0);
    }

    #[test]
    fn branch_network_reports_metrics() {
        let ifs = IFS::new(vec![
            TMA::from_translation(1.0, 0.0).with_probability(0.6),
            TMA::from_translation(-1.0, 0.0).with_probability(0.4),
        ])
        .expect("valid flow network");

        let mut network = BranchNetwork::new([0.0, 0.0]);
        network.grow_from_ifs(&ifs, &mut rand::thread_rng(), 1);

        assert!(network.max_depth() >= 1);
        assert!(network.total_flow() > 0.0);
        assert!(!network.children_of(0).is_empty());
    }

    #[test]
    fn branch_network_supports_graph_traversal_and_capacity_metrics() {
        let ifs = IFS::new(vec![
            TMA::from_translation(1.0, 0.0).with_probability(0.6),
            TMA::from_translation(-1.0, 0.0).with_probability(0.4),
        ])
        .expect("valid flow network");

        let mut network = BranchNetwork::new([0.0, 0.0]);
        network.grow_from_ifs(&ifs, &mut rand::thread_rng(), 2);

        let root_children = network.children_of(0);
        assert!(!root_children.is_empty());
        assert_eq!(network.parent_of(root_children[0]), Some(0));
        assert!(network.node_depth(root_children[0]).is_some());
        assert!(network.node_capacity(root_children[0]).is_some());
        assert!(network.total_capacity() >= network.total_flow());
        assert!(network.traverse_from(0).contains(&0));
        assert!(network.traverse_from(0).len() > root_children.len());
    }

    #[test]
    fn branch_network_exposes_flow_summary_metrics() {
        let ifs = IFS::new(vec![
            TMA::from_translation(1.0, 0.0).with_probability(0.6),
            TMA::from_translation(-1.0, 0.0).with_probability(0.4),
        ])
        .expect("valid flow network");

        let mut network = BranchNetwork::new([0.0, 0.0]);
        network.grow_from_ifs(&ifs, &mut rand::thread_rng(), 2);

        let summary = network.flow_summary();
        assert!(!summary.is_empty());
        assert!(summary[0].utilization >= 0.0);
        assert!(summary.iter().all(|entry| entry.capacity >= entry.flow));
    }
}
