use std::{fs, path::Path};

#[allow(unused)]
pub fn get_answer_p1(file: &str) -> i32 {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let filepath = Path::new(project_root).join(file);

    // data notes
    // no integers greater than 99 (small size)

    // iterate through string
    // set flag to indicate when ordering rules end and orders begin (`\n\n` delimiter)
    // order rules as DAG with Vec<Vec<uxize>> since 10-99 indices.
    // orderings should be stored as vectors.

    // order rules should be sorted topologically using DFS for efficient lookup.
    let input_str = fs::read_to_string(filepath).expect("Failed to read file to string.");

    // Initialize empty Vec<Vec<usize>> for both graph and orderings
    let mut graph: Vec<Vec<usize>> = Vec::new();
    let mut orderings: Vec<Vec<usize>> = Vec::new();

    // for_each() better than map() b/c no transformation or new return value
    input_str.split("\n\n").for_each(|section| {
        // graph edge section
        if section.contains('|') {
            let edges = section
                .lines()
                .map(|node_edge| {
                    node_edge
                        .split('|')
                        .map(|node| node.parse::<usize>().expect("Failed node to usize."))
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();
            graph.extend(edges);
        } else {
            // order section section
            let orders = section
                .lines()
                .map(|order| {
                    order
                        .split(',')
                        .map(|node| {
                            node.parse::<usize>()
                                .expect("Failed to convert node to usize.")
                        })
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();
            orderings.extend(orders);
        }
    });

    // Example of printing the graph and orderings
    println!("Graph:");
    for edge in &graph {
        println!("{:?}", edge);
    }

    println!("\nOrderings:");
    for order in &orderings {
        println!("{:?}", order);
    }

    1
}

#[allow(unused)]
pub fn get_answer_p2(file: &str) -> i32 {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let input_path = Path::new(project_root).join(file);
    2
}
