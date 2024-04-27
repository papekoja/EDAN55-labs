use crate::graph_parsing::parse_td::parse_td;
use crate::graph_parsing::parse_gr::parse_gr;


pub fn parse_gr_and_tr(filename: String) {
    let graph = parse_gr(&(filename.clone() + ".gr"));
    let tree = parse_td(&(filename + ".td"));

}