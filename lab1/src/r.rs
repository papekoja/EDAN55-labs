use std::collections::HashMap;
use rand::Rng;

pub fn r(nodes: HashMap<i32, Vec<(i32, i32)>>) {
    
}

fn coin_flip() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..2)
}