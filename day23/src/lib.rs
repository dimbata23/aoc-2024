use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::BufRead;

type UndirGraph = HashMap<String, HashSet<String>>;

#[derive(Eq, PartialEq, Debug, Hash)]
struct Triangle {
    a: String,
    b: String,
    c: String,
}

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    println!("Part one result: {res_part1}");

    let res_part2 = calculate_part2(&input);
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &[(String, String)]) -> usize {
    let graph = to_undir_graph(input);
    let triangles: HashSet<Triangle> = graph
        .keys()
        .filter(|node| node.starts_with('t'))
        .map(|node| get_triangles_from(&graph, node))
        .into_iter()
        .flat_map(|set| set)
        .collect();
    triangles.len()
}

fn calculate_part2(input: &[(String, String)]) -> String {
    let graph = to_undir_graph(input);
    let max_clique_size = max_node_degree(&graph);
    let max = find_largest_clique(&graph, max_clique_size);
    let mut max = max.clone().into_iter().collect::<Vec<_>>();
    max.sort();
    max.join(",")
}

fn find_largest_clique(graph: &UndirGraph, max_size: usize) -> HashSet<String> {
    let p = graph.keys().cloned().collect();
    let mut max_clique = HashSet::new();
    bron_kerbosch(
        graph,
        HashSet::new(),
        p,
        HashSet::new(),
        &mut max_clique,
        max_size,
    );
    max_clique
}

fn max_node_degree(graph: &UndirGraph) -> usize {
    graph.values().map(HashSet::len).max().unwrap_or(0)
}

fn to_undir_graph(edges: &[(String, String)]) -> UndirGraph {
    let mut graph = UndirGraph::new();
    for (a, b) in edges {
        graph.entry(a.clone()).or_default().insert(b.clone());
        graph.entry(b.clone()).or_default().insert(a.clone());
    }
    graph
}

fn get_triangles_from(graph: &UndirGraph, node: &str) -> HashSet<Triangle> {
    let mut triangles = HashSet::new();
    if let Some(neighbours) = graph.get(node) {
        for second_node in neighbours {
            for (third_node, th_neighs) in graph {
                if third_node == node || third_node == second_node {
                    continue;
                }

                if th_neighs.contains(node) && th_neighs.contains(second_node) {
                    triangles.insert(Triangle::new(node, second_node, third_node));
                }
            }
        }
    }
    triangles
}

impl Triangle {
    fn new(a: &str, b: &str, c: &str) -> Self {
        let mut nodes = vec![a, b, c];
        nodes.sort();
        Self {
            a: nodes[0].to_string(),
            b: nodes[1].to_string(),
            c: nodes[2].to_string(),
        }
    }
}

fn parse_file(file_path: &str) -> io::Result<Vec<(String, String)>> {
    Ok(File::open(file_path)
        .map(io::BufReader::new)?
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let (left, right) = line.split_once('-').unwrap();
            (left.to_string(), right.to_string())
        })
        .collect())
}

// Blatant AI generated algo :(
fn bron_kerbosch(
    graph: &UndirGraph,
    r: HashSet<String>,
    p: HashSet<String>,
    x: HashSet<String>,
    max_clique: &mut HashSet<String>,
    max_size: usize,
) -> bool {
    if r.len() == max_size {
        *max_clique = r;
        return true; // Signal to stop further recursion
    }

    // If P and X are both empty, we've found a maximal clique
    if p.is_empty() && x.is_empty() {
        // Update the largest clique if the current one is larger
        if r.len() > max_clique.len() {
            *max_clique = r;
        }
        return false;
    }

    // Pivoting to reduce the size of P
    let pivot = if let Some(v) = (p.union(&x)).next() {
        v.clone()
    } else {
        return false;
    };

    let neighbors_of_pivot = graph.get(&pivot).unwrap();
    let p_without_neighbors_of_pivot = p.difference(neighbors_of_pivot);

    // Iterate over the nodes in P that are not neighbors of the pivot
    for v in p_without_neighbors_of_pivot.cloned().collect::<Vec<_>>() {
        let neighbors = graph.get(&v).unwrap();

        // Recursive call with updated sets
        let mut new_r = r.clone();
        new_r.insert(v.clone());
        let new_p = p.intersection(neighbors).cloned().collect::<HashSet<_>>();
        let new_x = x.intersection(neighbors).cloned().collect::<HashSet<_>>();

        // Recurse with the new sets
        if bron_kerbosch(graph, new_r, new_p, new_x, max_clique, max_size) {
            return true; // Stop further recursion if a clique of max_size is found
        }

        // Move v from P to X
        let mut new_x = x.clone();
        new_x.insert(v.clone());
        let new_p = p
            .difference(&vec![v.clone()].into_iter().collect())
            .cloned()
            .collect();
        if bron_kerbosch(graph, r.clone(), new_p, new_x, max_clique, max_size) {
            return true; // Stop further recursion if a clique of max_size is found
        }
    }

    false // Signal to continue recursion
}
