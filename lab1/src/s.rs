use std::collections::HashMap;

use crate::utils::{calculate_cut, swap};


pub fn s(nodes: &HashMap<i32, Vec<(i32, i32)>>) -> i32 {
    let mut set_a: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let mut set_b: HashMap<i32, Vec<(i32, i32)>> = nodes.clone();

    let mut did_swap: bool = true;
    while did_swap {
        did_swap = false;

        for &node_id in nodes.keys() {
            let pre_cut = calculate_cut(&set_a, &set_b);
            swap(node_id, &mut set_a, &mut set_b);
            let post_cut = calculate_cut(&set_a, &set_b);
    
            if pre_cut > post_cut {
                swap(node_id, &mut set_a, &mut set_b);
            } else {
                did_swap = true;
            }
        }
    }

    calculate_cut(&set_a, &set_b)
}