use std::collections::{HashMap, HashSet};
use std::{fs, path::Path};

#[allow(unused)]
pub fn get_answer_p1(file: &str) -> i32 {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let filepath = Path::new(project_root).join(file);

    // TODO: Consider a buffered read instead.
    let input_str = fs::read_to_string(filepath).expect("Failed to read file to string.");

    // Initialize graph and orderings data structures
    let mut edges: Vec<Vec<usize>> = Vec::new();
    let mut orderings: Vec<Vec<usize>> = Vec::new();

    // Parse input and populate data structures
    input_str.split("\n\n").for_each(|section| {
        // graph section
        if section.contains('|') {
            section.lines().for_each(|pair| {
                let edge: Vec<usize> = pair
                    .split('|')
                    .map(|node| {
                        node.parse::<usize>()
                            .expect("Failed converting node to usize.")
                    })
                    .collect();
                assert_eq!(2, edge.len(), "Each edge should specify exactly 2 nodes");
                edges.push(edge);
            });
        } else {
            // orderings section
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
    println!("\nEdges:");
    for edge in &edges {
        println!("{:?}", edge);
    }

    println!("\nOrderings:");
    for ordering in &orderings {
        println!("In order {:?}", ordering);
    }

    let mut answer: i32 = 0;
    for ordering in orderings {
        if dfs_order_valid(&edges, &ordering) {
            answer += get_middle(&ordering);
        }
    }

    answer
}

#[allow(unused)]
pub fn get_answer_p2(file: &str) -> i32 {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let input_path = Path::new(project_root).join(file);
    2
}

fn dfs_order_valid(edges: &Vec<Vec<usize>>, ordering: &[usize]) -> bool {
    let mut stack: Vec<usize> = Vec::with_capacity(ordering.len());
    let mut visited: HashSet<usize> = HashSet::with_capacity(ordering.len());
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    // populate graph with relevant rules
    for edge in edges {
        let origin = edge[0];
        let destination = edge[1];
        if ordering.contains(&origin) {
            graph.entry(origin).or_default().push(destination);
        }
    }

    println!("Graph:");
    for edge in &graph {
        println!("{:?}", edge);
    }

    for node in ordering {
        if !visited.contains(node) && !dfs_helper(&graph, *node, &mut visited, &mut stack) {
            println!("node {} is false (top)", node);
            return false;
        }
    }
    true
}

fn dfs_helper(
    graph: &HashMap<usize, Vec<usize>>,
    node: usize,
    visited: &mut HashSet<usize>,
    stack: &mut Vec<usize>,
) -> bool {
    // if node is already in stack, there's a cycle (invalid)
    if stack.contains(&node) {
        return false;
    };
    stack.push(node);

    // check dependencies of current node
    if let Some(dependencies) = graph.get(&node) {
        for dependency in dependencies {
            if !visited.contains(dependency) && !dfs_helper(graph, node, visited, stack) {
                println!("dependency {} is false (recurse)", dependency);
                return false;
            }
        }
    }
    true
}

fn get_middle(ordering: &[usize]) -> i32 {
    ((ordering.len() / 2) + 1) as i32
}
