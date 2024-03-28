use std::collections::HashMap;

use crate::utils::{calculate_cut, get_random_subset};

pub fn r(nodes: &HashMap<i32, Vec<(i32, i32)>>) -> i32 {
    let (set_a, set_b) = get_random_subset(nodes);
    calculate_cut(&set_a, &set_b)
}