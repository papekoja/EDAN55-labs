use graph_parsing::{parse_gr::parse_gr, parse_td::parse_td};

use crate::algorithm::algorithm;

mod algorithm;
mod graph_parsing;
mod models;

fn main() {
    let filename = "web4".to_string();
    let graph = parse_gr(&(filename.clone() + ".gr"));
    let tree = parse_td(&(filename + ".td"));

    println!("======== .gr ========");
    println!("{:?}", graph);

    println!("======== .td ========");
    tree.print_tree();

    algorithm(&graph, &tree);
}
