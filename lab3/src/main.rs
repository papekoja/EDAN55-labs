mod graph_parsing;
mod models;
use models::arena_tree::ArenaTree;

fn main() {
    let mut tree = ArenaTree::<String>::default();

    // Add nodes
    let root_idx = tree.node("Root".to_string());
    let child1_idx = tree.node("Child1".to_string());
    let child2_idx = tree.node("Child2".to_string());
    let subchild1_idx = tree.node("SubChild1".to_string());

    // Establish relationships
    tree.arena[root_idx].children.push(child1_idx);
    tree.arena[child1_idx].parent = Some(root_idx);
    tree.arena[child1_idx].children.push(subchild1_idx);
    tree.arena[subchild1_idx].parent = Some(child1_idx);
    tree.arena[root_idx].children.push(child2_idx);
    tree.arena[child2_idx].parent = Some(root_idx);

    // Print the tree
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