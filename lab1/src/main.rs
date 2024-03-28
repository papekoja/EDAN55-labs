use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path_matching = "../data/matching_1000.txt";
    //let path_pw = "../data/pw09_100.9.txt";
    let file = File::open(path_matching).unwrap();
    let mut reader = BufReader::new(file);
    
    let mut edges = Vec::new();
    
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if index == 0 {
            continue;
        }

        let parts: Vec<(&str)> = line.split_whitespace().collect();
        let edge = vec![
            
        ];
    }
}