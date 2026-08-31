use tma_engine::geometry::{BranchNetwork, IFS, TMA};

fn build_branch_ifs() -> IFS {
    IFS::new(vec![
        TMA::new([[0.83, 0.0], [0.0, 0.83]], [0.0, 0.12]).with_probability(0.55),
        TMA::new([[0.77, 0.18], [-0.18, 0.77]], [0.12, 0.16]).with_probability(0.22),
        TMA::new([[0.77, -0.18], [0.18, 0.77]], [-0.12, 0.16]).with_probability(0.23),
    ])
    .expect("branching IFS should be valid")
}

fn summarize_branches(depth_limit: usize) -> Vec<(usize, usize, f64, f64)> {
    let ifs = build_branch_ifs();
    let mut rng = rand::thread_rng();
    let mut network = BranchNetwork::new([0.0, 0.0]);
    network.grow_from_ifs(&ifs, &mut rng, depth_limit);

    network
        .flow_summary()
        .into_iter()
        .map(|entry| (entry.node_index, entry.depth, entry.flow, entry.capacity))
        .collect()
}

fn main() {
    let summary = summarize_branches(4);
    let total_nodes = summary.len();
    let max_depth = summary
        .iter()
        .map(|(_, depth, _, _)| *depth)
        .max()
        .unwrap_or(0);
    let total_flow = summary.iter().map(|(_, _, flow, _)| *flow).sum::<f64>();

    println!("Branch summary example");
    println!("Total nodes: {}", total_nodes);
    println!("Maximum depth: {}", max_depth);
    println!("Total flow: {:.4}", total_flow);
    println!("First 12 entries:");

    for (node_index, depth, flow, capacity) in summary.iter().take(12) {
        println!(
            "  node={} depth={} flow={:.4} capacity={:.4}",
            node_index, depth, flow, capacity
        );
    }
}
