//Huffman tree
use std::{
    cmp::Ordering,
    cmp::Ordering::Equal,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    fs,
};

#[derive(Debug)]
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,

    val: Option<char>,
    freq: usize,
}

impl Node {
    pub fn new_node(val: Option<char>, freq: usize) -> Self {
        Self {
            left: None,
            right: None,

            val,
            freq,
        }
    }

    pub fn new_parent(left: Self, right: Self) -> Self {
        let freq = left.freq + right.freq;
        Self {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),

            val: None,
            freq,
        }
    }

    pub fn is_leaf(&self) -> bool {
        match (&self.left, &self.right) {
            (None, None) => true,
            _ => false,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.freq.cmp(&other.freq))
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.freq.cmp(&other.freq) {
            Equal => {
                if let (Some(v1), Some(v2)) = (self.val, other.val) {
                    v1.cmp(&v2)
                } else {
                    Equal
                }
            }
            other => other,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node: {} {}", self.val.unwrap(), self.freq)
    }
}

#[derive(Debug)]
pub struct NodeArray {
    n_char: usize,
    capacity: usize,
    nodes: Vec<Node>,
}

impl NodeArray {
    pub fn new_from_file(src: &str, freq: &mut HashMap<char, usize>) -> Self {
        //file IO
        let binding = fs::read_to_string(src).unwrap();
        let content = binding.chars();
        let mut nodes: Vec<Node> = vec![];

        for c in content {
            freq.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }

        for (k, v) in freq.iter() {
            let n = Node::new_node(Some(*k), *v);
            nodes.push(n);
        }

        nodes.sort();

        Self {
            n_char: nodes.len(),
            capacity: 256,
            nodes,
        }
    }

    pub fn build_huffman_tree(&self) -> Self {
        let mut heap = BinaryHeap::new();

        Self {
            n_char: 0,
            capacity: 256,
            nodes: heap.into_vec(),
        }
    }
}
