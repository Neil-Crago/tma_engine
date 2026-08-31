use std::collections::VecDeque;

use tma_engine::geometry::{IFS, TMA};

#[derive(Clone, Debug)]
struct BranchNode {
    point: [f64; 2],
    depth: usize,
}

fn build_branching_ifs() -> IFS {
    let transforms = vec![
        TMA::new([[0.77, 0.0], [0.0, 0.77]], [0.0, 0.12]).with_probability(0.52),
        TMA::new([[0.72, 0.18], [-0.18, 0.72]], [0.12, 0.18]).with_probability(0.24),
        TMA::new([[0.72, -0.18], [0.18, 0.72]], [-0.12, 0.18]).with_probability(0.24),
    ];

    IFS::new(transforms).expect("branching IFS should be valid")
}

fn simulate_vascular_network(iterations: usize) -> Vec<BranchNode> {
    let ifs = build_branching_ifs();
    let mut rng = rand::thread_rng();
    let mut queue = VecDeque::from([BranchNode {
        point: [0.0, 0.0],
        depth: 0,
    }]);
    let mut output = Vec::new();

    while let Some(node) = queue.pop_front() {
        output.push(node.clone());

        if node.depth >= iterations {
            continue;
        }

        for _ in 0..2 {
            let transform = ifs.choose_transformation(&mut rng);
            let projected = transform.apply(node.point);
            queue.push_back(BranchNode {
                point: projected,
                depth: node.depth + 1,
            });
        }
    }

    output
}

fn main() {
    let network = simulate_vascular_network(8);

    let max_depth = network.iter().map(|node| node.depth).max().unwrap_or(0);
    let total_points = network.len();
    let average_radius = network
        .iter()
        .map(|node| node.point[0].hypot(node.point[1]))
        .sum::<f64>()
        / total_points as f64;

    println!("Vascular-like branching sample");
    println!("Total nodes: {}", total_points);
    println!("Maximum depth: {}", max_depth);
    println!("Average radius: {:.4}", average_radius);
    println!("First 10 nodes:");

    for node in network.iter().take(10) {
        println!("  depth={} point={:?}", node.depth, node.point);
    }
}
