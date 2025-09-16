use std::ops::Mul;

use rand::Rng;

// A point in our 2D fractal space.
pub type Point = [f64; 2];

/// TMA: Transformation, Matrix, Affine.
/// Represents a 2D affine transformation T(v) = A*v + c.
/// This is a fundamental building block for Iterated Function Systems (IFS).
#[derive(Debug, Clone, Copy)]
pub struct TMA {
    /// The 2x2 matrix 'A' for linear transformations (rotation, scaling, shear).
    pub matrix: [[f64; 2]; 2],

    /// The 2D vector 'c' for translation.
    pub vector: Point,

    /// Optional probability for use in stochastic IFS (e.g., the Chaos Game).
    /// The sum of probabilities of all TMAs in a system should equal 1.0.
    pub probability: Option<f64>,
}

impl TMA {
    /// Creates a new TMA from a given matrix and vector.
    pub fn new(matrix: [[f64; 2]; 2], vector: Point) -> Self {
        TMA {
            matrix,
            vector,
            probability: None,
        }
    }

    /// Creates an identity transformation (no change).
    pub fn identity() -> Self {
        TMA {
            matrix: [[1.0, 0.0], [0.0, 1.0]],
            vector: [0.0, 0.0],
            probability: None,
        }
    }

    /// A helper to create a uniform scaling transformation.
    pub fn from_scale(s: f64) -> Self {
        TMA::new([[s, 0.0], [0.0, s]], [0.0, 0.0])
    }

    /// A helper to create a translation.
    pub fn from_translation(tx: f64, ty: f64) -> Self {
        TMA::new([[1.0, 0.0], [0.0, 1.0]], [tx, ty])
    }

    /// A helper to create a rotation transformation (counter-clockwise).
    /// `theta` is in radians.
    pub fn from_rotation(theta: f64) -> Self {
        let (sin_t, cos_t) = theta.sin_cos();
        TMA::new([[cos_t, -sin_t], [sin_t, cos_t]], [0.0, 0.0])
    }

    /// Attaches a probability to the transformation.
    pub fn with_probability(mut self, p: f64) -> Self {
        self.probability = Some(p);
        self
    }

    /// Applies the transformation to a point.
    /// Returns a new, transformed point.
    pub fn apply(&self, p: Point) -> Point {
        let x = p[0];
        let y = p[1];

        let new_x = self.matrix[0][0] * x + self.matrix[0][1] * y + self.vector[0];
        let new_y = self.matrix[1][0] * x + self.matrix[1][1] * y + self.vector[1];

        [new_x, new_y]
    }

    /// Composes this transformation with another one.
    /// This is equivalent to `self * other`, which applies `other` first, then `self`.
    pub fn compose(&self, other: &TMA) -> Self {
        // New matrix A_new = A_self * A_other
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

        // New vector c_new = A_self * c_other + c_self
        let c1 = self.vector;
        let c2 = other.vector;
        let new_vector = [
            m1[0][0] * c2[0] + m1[0][1] * c2[1] + c1[0],
            m1[1][0] * c2[0] + m1[1][1] * c2[1] + c1[1],
        ];

        TMA::new(new_matrix, new_vector)
    }
}

// Operator Overloading for Algebraic Composition
// TMA * TMA -> TMA (Composition)
impl Mul<TMA> for TMA {
    type Output = TMA;

    fn mul(self, rhs: TMA) -> Self::Output {
        self.compose(&rhs)
    }
}

// Operator Overloading for Application
// TMA * Point -> Point (Transformation)
impl Mul<Point> for TMA {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        self.apply(rhs)
    }
}

/// A collection of TMA transformations that defines a fractal.
pub struct IFS {
    transformations: Vec<TMA>,
    // We can pre-calculate cumulative probabilities for efficient selection.
    cumulative_probs: Vec<f64>,
}
impl IFS {
    /// Creates a new IFS from a vector of TMAs.
    /// It calculates the cumulative probabilities for the Chaos Game.
    pub fn new(transformations: Vec<TMA>) -> Result<Self, &'static str> {
        if transformations.is_empty() {
            return Err("IFS cannot be empty.");
        }

        let mut cumulative_probs = Vec::with_capacity(transformations.len());
        let mut total_prob = 0.0;

        for tma in &transformations {
            // Ensure every TMA has a probability set.
            let prob = tma
                .probability
                .ok_or("All TMAs in an IFS must have a probability for stochastic generation.")?;
            total_prob += prob;
            cumulative_probs.push(total_prob);
        }

        // It's good practice to ensure probabilities sum to ~1.0
        if (total_prob - 1.0).abs() > 1e-6 {
            return Err("Probabilities of all TMAs must sum to 1.0.");
        }

        Ok(IFS {
            transformations,
            cumulative_probs,
        })
    }

    /// Chooses a transformation based on their weighted probabilities.
    pub fn choose_transformation(&self) -> &TMA {
        let mut rng = rand::thread_rng();
        let r: f64 = rng.gen_range(0.0..1.0); // A random float between 0.0 and 1.0

        // Find which "bin" the random number falls into.
        for (i, &cumulative_prob) in self.cumulative_probs.iter().enumerate() {
            if r < cumulative_prob {
                return &self.transformations[i];
            }
        }

        // Fallback to the last one, in case of floating point inaccuracies.
        self.transformations.last().unwrap()
    }

    /// Runs the Chaos Game to generate a set of points for the fractal.
    // In the impl IFS block
    // The return type is now Vec<(Point, usize)>
    pub fn run_chaos_game(
        &self,
        num_points: usize,
        warmup_iterations: usize,
    ) -> Vec<(Point, usize)> {
        let mut points = Vec::with_capacity(num_points);
        let mut current_point: Point = [0.0, 0.0];

        // We need to track the chosen index
        let mut chosen_index = 0;

        let total_iterations = num_points + warmup_iterations;

        for i in 0..total_iterations {
            // We'll rewrite the selection logic slightly to get the index.
            let mut rng = rand::thread_rng();
            let r: f64 = rng.gen_range(0.0..1.0);
            for (j, &cumulative_prob) in self.cumulative_probs.iter().enumerate() {
                if r < cumulative_prob {
                    chosen_index = j;
                    break;
                }
            }
            let tma = &self.transformations[chosen_index];
            current_point = tma.apply(current_point);

            if i >= warmup_iterations {
                points.push((current_point, chosen_index));
            }
        }
        points
    }
}
