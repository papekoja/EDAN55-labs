use graph_parsing::{parse_gr::parse_gr, parse_td::parse_td};

mod algorithm;
mod graph_parsing;
mod models;

fn main() {
    let filename = "BalancedTree_3_5".to_string();
    let graph = parse_gr(&(filename.clone() + ".gr"));
    let tree = parse_td(&(filename + ".td"));

    println!("======== .gr ========");
    println!("{:?}", graph);

    println!("======== .td ========");
    tree.print_tree();
}

// To find a maximum weight independent set of G:
fn _f_t() {
    // given a tree decomposition (T, {Vt}) of G:
    //Modify the tree decomposition if necessary so that it is nonredundant
    //Root T at a node r
    //for each node t of T in post-order
    //  if t is a leaf then
    //      for each independent set U of Vt
    //          f_t(U)=w(U)
    //  Else
    //      for each independent set U of Vt
    //          f_t(U) is determined by the recurrence in (10.8)
    //return max{f_r(U): U subs V_r is independent}
}

/*
Gör en adjacency vektor för alla noder i en bag. (ettor och nollor)
Gör sedan alla permutationer av alla sets och solla ut alla som inte är independent set.
Det görs genom att göra AND mellan alla noder som är med i ett set genom adjacency matrixen, ifall det blir någon etta så är det inte
ett independent set.

så för detta har man alltså en adjacency matrix och en hashmap med independent sets.
*/