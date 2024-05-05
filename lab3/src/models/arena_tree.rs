/* Code from: https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6 */

use std::collections::HashMap;

#[derive(Debug)]
pub struct Node<T>
where
    T: PartialEq,
{
    pub idx: usize,
    pub val: T,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    // key: global index, value: local index
    pub global_to_local: HashMap<usize, usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    pub fn new(idx: usize, val: T, global_to_local: HashMap<usize, usize>) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
            global_to_local,
        }
    }
}

#[derive(Debug, Default)]
pub struct ArenaTree<T>
where
    T: PartialEq,
{
    pub arena: HashMap<usize, Node<T>>,
    pub leafs: Vec<usize>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq + std::fmt::Debug,
{
    pub fn insert_node(&mut self, idx: usize, val: T, global_to_local: HashMap<usize, usize>) {
        let new_node = Node::new(idx, val, global_to_local);
        if let Some(node) = self.arena.get_mut(&idx) {
            *node = new_node;
            eprintln!("Warning: inserting node to already existing");
        } else {
            self.arena.insert(idx, new_node);
        }
    }

    pub fn size(&self) -> usize {
        self.arena.len()
    }

    pub fn edges(&self) -> usize {
        self.arena
            .iter()
            .fold(0, |acc, (&key, node)| acc + node.children.len())
    }

    pub fn depth(&self, idx: usize) -> usize {
        match self.arena[&idx].parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }

    pub fn print_tree(&self) {
        println!(
            "Tree contains {} nodes and {} edges.\nLeafs: {:?}",
            self.size(),
            self.edges(),
            self.leafs
        );
        for (&index, node) in &self.arena {
            if node.parent.is_none() {
                // Start recursive print from root nodes
                self.print_node(node.idx, 0);
            }
        }
    }

    // Recursive function to print a node and its children
    fn print_node(&self, idx: usize, depth: usize) {
        let node = &self.arena[&idx];
        let indent = " ".repeat(depth * 4); // Create an indent based on depth for readability
        println!(
            "{}Node idx: {}, val: {:?}, parent: {:?}, children: {:?}",
            indent, node.idx, node.val, node.parent, node.children
        );

        // Recursive call to print each child
        for &child_idx in &node.children {
            self.print_node(child_idx, depth + 1);
        }
    }
}
