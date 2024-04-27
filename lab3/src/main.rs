use graph_parsing::{parse_gr::parse_gr, parse_td::parse_td};

mod graph_parsing;
mod models;

fn main() {
    let filename = "TutteGraph".to_string();
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
Inläsning:
börja läs in träden. varje nod i trädet är en bag.
en nod typ har en förälder och lista med barn, index, och lista med noder
ha sedan en adjacency med alla noder separat. välj en nod som rot, sätt alla dess grannar från adjacency grafen till föräldrar och barn

*/