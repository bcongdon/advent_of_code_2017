use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

fn parse_graph(edges: Vec<&str>) -> HashMap<String, Vec<&str>> {
    let mut graph = HashMap::new();

    for e in edges.iter() {
        let mut vertices: Vec<&str> = e.split(" ").collect::<Vec<_>>();

        let source_node = String::from(vertices[0]);
        let mut curr_edges: Vec<&str> = Vec::new();
        for v in vertices[2..].iter() {
            let neighbor = v;
            curr_edges.push(&neighbor);
        }

        graph.insert(source_node, curr_edges);
    }

    graph
}

fn bfs_reach(graph: &HashMap<String, Vec<&str>>, start: &str) -> HashSet<String> {
    let mut to_visit: VecDeque<String> = VecDeque::new();
    to_visit.push_back(String::from(start));

    let mut reachable: HashSet<String> = HashSet::new();
    reachable.insert(String::from(start));

    while !to_visit.is_empty() {
        let curr = to_visit.pop_back().unwrap().replace(",", "");
        for neighbor in graph.get(&curr).unwrap().iter() {
            if !reachable.contains(*neighbor) {
                reachable.insert(String::from(*neighbor));
                to_visit.push_back(String::from(*neighbor));
            }
        }
    }

    reachable
}

fn num_connected_components(graph: HashMap<String, Vec<&str>>) -> u32 {
    let mut unmarked = graph.keys().collect::<HashSet<_>>();

    let mut components = 0;
    while !unmarked.is_empty() {
        components += 1;
        let component = bfs_reach(&graph, unmarked.iter().next().unwrap());
        component.iter().for_each(|node_label| {
            unmarked.remove(node_label);
        });
    }

    components
}

pub fn main() {
    let mut f = File::open("12.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents = contents.replace(",", "");

    let edges = contents.split("\n").collect::<Vec<_>>();
    let graph = parse_graph(edges);

    println!("Part 1: {}", bfs_reach(&graph, "0").len());
    println!("Part 2: {}", num_connected_components(graph));
}
