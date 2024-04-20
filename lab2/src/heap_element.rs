use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
pub struct HeapElement {
    pub index: usize,
    pub value: usize,
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.value, other.value) {
            (0,0) => self.index.cmp(&other.index),
            (0, _) => Ordering::Greater,
            (_, 0) => Ordering::Less,
            _ => other.value.cmp(&self.value),
        }
    }
}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}