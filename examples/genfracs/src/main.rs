use tma_engine::geometry::{IFS, TMA};
use tma_engine::render::Renderer;

fn get_sierpinski_ifs() -> Vec<TMA> {
    let p = 1.0 / 3.0; // Equal probability for each transformation

    // T1: Scale by 0.5, translate to bottom-left
    let t1 = TMA::new([[0.5, 0.0], [0.0, 0.5]], [0.0, 0.0]).with_probability(p);

    // T2: Scale by 0.5, translate to bottom-right
    let t2 = TMA::new([[0.5, 0.0], [0.0, 0.5]], [0.5, 0.0]).with_probability(p);

    // T3: Scale by 0.5, translate to top-middle
    let t3 = TMA::new([[0.5, 0.0], [0.0, 0.5]], [0.25, 0.5]).with_probability(p);

    vec![t1, t2, t3]
}

// A new function to define the fern's transformations.
fn get_barnsley_fern_ifs() -> Vec<TMA> {
    let t1 = TMA::new([[0.0, 0.0], [0.0, 0.16]], [0.0, 0.0]).with_probability(0.01);
    let t2 = TMA::new([[0.85, 0.04], [-0.04, 0.85]], [0.0, 1.6]).with_probability(0.85);
    let t3 = TMA::new([[0.2, -0.26], [0.23, 0.22]], [0.0, 1.6]).with_probability(0.07);
    let t4 = TMA::new([[-0.15, 0.28], [0.26, 0.24]], [0.0, 0.44]).with_probability(0.07);

    vec![t1, t2, t3, t4]
}


fn do_sierpinski() {
    println!("Fractal Algebra Workspace: Sierpinski Triangle session.");

    // 1. Define the IFS for the Sierpinski Triangle
    let sierpinski_tmas = get_sierpinski_ifs();
    let sierpinski_ifs = IFS::new(sierpinski_tmas).expect("Failed to create Sierpinski IFS.");

    // 2. Run the Chaos Game
    let num_points = 100_000;
    let warmup = 100;
    println!("Generating {} points for the Sierpinski Triangle...", num_points);
    let fractal_data = sierpinski_ifs.run_chaos_game(num_points, warmup);
    println!("Generation complete.");

    // 3. Render the points to a file
    let renderer = Renderer::new(2048, 2048);
    let filename = "sierpinski_triangle.png";
    println!("Rendering image to {}...", filename);

    // The render function now expects the data with color info
    match renderer.render(&fractal_data, filename) {
        Ok(_) => println!("Successfully saved image."),
        Err(e) => eprintln!("Error saving image: {}", e),
    }
}

fn do_barnsley_fern() {
    println!("Fractal Algebra Workspace: Barnsley Fern session.");

    // 1. Define the IFS for the Barnsley Fern
    let fern_tmas = get_barnsley_fern_ifs();
    let fern_ifs = IFS::new(fern_tmas).expect("Failed to create Barnsley Fern IFS.");

    // 2. Run the Chaos Game
    let num_points = 100_000;
    let warmup = 100;
    println!("Generating {} points for the Barnsley Fern...", num_points);
    let fractal_data = fern_ifs.run_chaos_game(num_points, warmup);
    println!("Generation complete.");

    // 3. Render the points to a file
    let renderer = Renderer::new(2048, 2048);
    let filename = "barnsley_fern.png";
    println!("Rendering image to {}...", filename);

    // The render function now expects the data with color info
    match renderer.render(&fractal_data, filename) {
        Ok(_) => println!("Successfully saved image."),
        Err(e) => eprintln!("Error saving image: {}", e),
    }
}

fn main() {
    do_sierpinski();
    do_barnsley_fern();
}
