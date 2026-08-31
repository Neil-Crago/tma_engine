use std::collections::VecDeque;

use tma_engine::geometry::{IFS, TMA};

#[derive(Clone, Debug)]
struct FlowNode {
    point: [f64; 2],
    depth: usize,
    flow: f64,
}

fn build_flow_ifs() -> IFS {
    let transforms = vec![
        TMA::new([[0.82, 0.05], [-0.05, 0.82]], [0.0, 0.12]).with_probability(0.45),
        TMA::new([[0.78, 0.15], [-0.12, 0.78]], [0.18, 0.08]).with_probability(0.28),
        TMA::new([[0.75, -0.15], [0.12, 0.75]], [-0.18, 0.08]).with_probability(0.27),
    ];

    IFS::new(transforms).expect("flow IFS should be valid")
}

fn simulate_flow_network(iterations: usize) -> Vec<FlowNode> {
    let ifs = build_flow_ifs();
    let mut rng = rand::thread_rng();
    let mut queue = VecDeque::from([FlowNode {
        point: [0.0, 0.0],
        depth: 0,
        flow: 1.0,
    }]);
    let mut output = Vec::new();

    while let Some(node) = queue.pop_front() {
        output.push(node.clone());

        if node.depth >= iterations {
            continue;
        }

        let mut next_flow = node.flow;
        for _ in 0..2 {
            let transform = ifs.choose_transformation(&mut rng);
            let projected = transform.apply(node.point);
            next_flow *= 0.94;
            queue.push_back(FlowNode {
                point: projected,
                depth: node.depth + 1,
                flow: next_flow,
            });
        }
    }

    output
}

fn main() {
    let flow = simulate_flow_network(9);
    let total_nodes = flow.len();
    let max_depth = flow.iter().map(|node| node.depth).max().unwrap_or(0);
    let average_flow = flow.iter().map(|node| node.flow).sum::<f64>() / total_nodes as f64;

    println!("Recursive flow network sample");
    println!("Total nodes: {}", total_nodes);
    println!("Maximum depth: {}", max_depth);
    println!("Average flow: {:.4}", average_flow);
    println!("First 12 nodes:");

    for node in flow.iter().take(12) {
        println!("  depth={} flow={:.4} point={:?}", node.depth, node.flow, node.point);
    }
}
