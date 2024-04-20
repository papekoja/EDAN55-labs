use std::collections::HashMap;
use std::env;
use std::fs;
use std::cmp::max;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1); // Exit the program with an error code
    }

    let file_path = &args[1];
    let contents: String = fs::read_to_string(file_path).expect("wowee");

    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();

    for (i, line) in contents.lines().enumerate().skip(1) {
        for (j, index) in line.split_whitespace().enumerate()  {
            let connection = index.parse::<u32>().unwrap();
            if connection == 1 {
                let vertex_u = (j + 1) as u32;
                let vertex_v = i as u32;
                graph.entry(vertex_u).or_insert_with(Vec::new).push(vertex_v);
            }
            // print!("{index}, ");
        }
        // println!()
    }
    let mut r_zero_graph = graph.clone();
    println!("R_0 = {}", r_zero(&mut r_zero_graph));
    
}


fn r_zero(graph: &mut HashMap<u32, Vec<u32>>) -> i32 {
    if graph.is_empty() {
        return 0;
    }

    
    let mut to_remove: Option<u32> = None;
    let mut neighbours: Option<Vec<u32>> = None;
    
    for (vertex, vertices) in graph.iter() {
        if graph.get(vertex).unwrap().len() == 0 {
            to_remove = Some(*vertex);
            neighbours = Some(vertices.clone());
            break;
        }

        
    }

    if let Some(vertex) = to_remove {        
        if let Some(vertices) = neighbours {
            remove_all_neighbours(graph, vertex, &vertices);
        } else {
            graph.remove(&vertex);
        }
        return 1 + r_zero(graph);
    }
    
    let (max_neighbours_vertex, neighbours) = graph.iter()
        .max_by_key(|(_, edges)| edges.len())
        .map(|(vertex, edges)| (*vertex, edges.clone()))
        .unwrap();

    let mut graph_minus_neighbours = graph.clone();
    remove_all_neighbours(&mut graph_minus_neighbours, max_neighbours_vertex, &neighbours);

    let mut graph_minus_vertex: HashMap<u32, Vec<u32>> = graph.clone();
    remove_all_neighbours(&mut graph_minus_vertex, max_neighbours_vertex, &vec![max_neighbours_vertex]);
    
    max(1 + r_zero(&mut graph_minus_neighbours), r_zero(&mut graph_minus_vertex))
    
}

fn r_one()

fn remove_all_neighbours(graph: &mut HashMap<u32, Vec<u32>>, vertex: u32, neighbours: &Vec<u32>) {
    graph.remove(&vertex);
    neighbours.iter().for_each(|&neighbour| {graph.remove(&neighbour);});
    for edges in graph.values_mut() {
        edges.retain(|&x| x != vertex && !neighbours.contains(&x));
    }
}

