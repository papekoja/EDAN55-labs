use std::collections::{HashMap, HashSet};
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
    let mut call_count: u32 = 0;
    println!("R algorithm = {:?}, {}", r_algorithm(&mut graph, &mut call_count), call_count);
}


fn r_algorithm(graph: &mut HashMap<u32, Vec<u32>>, call_count: &mut u32) -> i32 {
    *call_count += 1;
    if graph.is_empty() {
        return 0;
    }
    
    let mut v: Option<u32> = None;
    let mut v_edges: Option<Vec<u32>> = None;
    let z = graph.len() as u32 + 130;
    let mut z_edges: Vec<u32> = Vec::new();
    let mut add_z: bool = false;

    for (&vertex, vertices) in graph.iter() {
        match vertices.len() {
            /* R_0 Rule */
            0 => {
                v = Some(vertex);
                break;
            },
            /* R_1 Rule */
            1 => {
                v = Some(vertex);
                v_edges = Some(vertices.clone());
                break;
            },
            /* R_2 Rule */
            2 => {
                v = Some(vertex);
                v_edges = Some(vertices.clone());
                
                let u = vertices[0];
                let w = vertices[1];                    
                if graph.get(&u).unwrap().contains(&w) {
                    break;
                } else {
                    add_z = true;
                    let mut combined_neighbours: HashSet<u32> = HashSet::new();
                    let neighbours_of_u = graph.get(&u).unwrap();
                    let neighbours_of_w = graph.get(&w).unwrap();
                    combined_neighbours.extend(neighbours_of_u.iter().cloned());
                    combined_neighbours.extend(neighbours_of_w.iter().cloned());
                    combined_neighbours.remove(&vertex);
                    z_edges.extend(combined_neighbours.into_iter());      
                    break;
                }
            },
           _ => {}
        }
    }

    if let Some(vertex) = v {
        if add_z {
            graph.insert(z, z_edges.clone());
            for &vertex in &z_edges {
                graph.entry(vertex).or_insert_with(Vec::new).push(z);
            }
            remove_vertex_and_neighbours(graph, vertex, &v_edges.unwrap());
        }
        else if let Some(vertices) = v_edges {
            remove_vertex_and_neighbours(graph, vertex, &vertices);
        } else {
            graph.remove(&vertex);
        }
        return 1 + r_algorithm(graph, call_count);

    }

    /* Get the vertex with highest amount of neighbours by iterating through the graph */
    let (max_neighbours_vertex, neighbours) = graph.iter()
        .max_by_key(|(_, edges)| edges.len())
        .map(|(vertex, edges)| (*vertex, edges.clone()))
        .unwrap();

    let mut graph_minus_neighbours = graph.clone();
    remove_vertex_and_neighbours(&mut graph_minus_neighbours, max_neighbours_vertex, &neighbours);

    let mut graph_minus_vertex: HashMap<u32, Vec<u32>> = graph.clone();
    remove_vertex_and_neighbours(&mut graph_minus_vertex, max_neighbours_vertex, &vec![max_neighbours_vertex]);
    

    let result1 = r_algorithm(&mut graph_minus_neighbours, call_count);
    let result2 = r_algorithm(&mut graph_minus_vertex, call_count);

    max(result1 + 1, result2)

    
}

fn remove_vertex_and_neighbours(graph: &mut HashMap<u32, Vec<u32>>, vertex: u32, neighbours: &Vec<u32>) {
    graph.remove(&vertex);
    neighbours.iter().for_each(|&neighbour| {graph.remove(&neighbour);});
    for edges in graph.values_mut() {
        edges.retain(|&x| x != vertex && !neighbours.contains(&x));
    }
}
