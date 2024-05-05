use std::vec;

use graph_parsing::{parse_gr::parse_gr, parse_td::parse_td};

use crate::algorithm::algorithm;

mod algorithm;
mod graph_parsing;
mod models;

fn main() {
    let filenames = vec![
        "web4",
        "WorldMap",
        "FibonacciTree_10",
        "StarGraph_100",
        "TutteGraph",
        "DorogovtsevGoltsevMendesGraph",
        "HanoiTowerGraph_4_3",
    ];
    for filename in filenames {
        println!("======== {} ========", filename);
        let graph = parse_gr(&(filename.to_owned() + ".gr"));
        let tree = parse_td(&(filename.to_string() + ".td"));

        // println!("======== .gr ========");
        // println!("{:?}", graph);

        // println!("======== .td ========");
        // tree.print_tree();

        algorithm(&graph, &tree);
    }
}
