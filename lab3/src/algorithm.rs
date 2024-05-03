use std::collections::HashMap;

use crate::models::arena_tree::ArenaTree;

pub fn algorithm(graph: &HashMap<usize, Vec<usize>>, tree: &ArenaTree<Vec<usize>>) {
    let adj_matrix = get_adjacency_matrix(graph);
    // println!("Adjacency matrix: ");
    // for (node, neighbours) in adj_matrix {
    //     println!("{}: {:b}", node, neighbours)
    // }
    
}

fn get_adjacency_matrix(graph: &HashMap<usize, Vec<usize>>) -> HashMap<usize, u128> {
    let mut adj_matrix: HashMap<usize, u128> = HashMap::new();
    for (node, neighbours) in graph.iter() {
        adj_matrix.insert(*node, 0);
        neighbours.iter().for_each(|n| {
            let adj_bits = adj_matrix.get_mut(node).unwrap();
            let neighbour_bit: u128 = 1 << n;
            *adj_bits = *adj_bits | neighbour_bit;
        });
    }
    adj_matrix
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
Gör sedan alla permutationer av alla setet i bagen och solla ut alla som inte är independent set:
Det görs genom att göra AND mellan
•en vektor där alla noder som är med i setet är 1
•och adjacency vektorn för de noder som är med i setet
Ifall det finns en etta någonstans är det inte ett inependent set.

För detta har man alltså en hashmap med independent sets.
*/