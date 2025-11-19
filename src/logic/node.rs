use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Node {
    pub freq: usize,
    pub value: Option<u8>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new_leaf(freq: usize, value: u8) -> Self {
        Node {
            freq,
            value: Some(value),
            left: None,
            right: None,
        }
    }

    pub fn new_internal(freq: usize, left: Node, right: Node) -> Self {
        Node {
            freq,
            value: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
