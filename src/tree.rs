//Huffman tree
use std::{
    collections::{BinaryHeap, HashMap},
    fs,
};

#[derive(Debug)]
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,

    val: Option<char>,
    freq: Option<usize>,
}

impl Node {
    pub fn new_node(val: char, freq: usize) -> Self {
        Self {
            left: None,
            right: None,

            val: Some(val),
            freq: Some(freq),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq.unwrap() == other.freq.unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let (Some(f1), Some(f2)) = (self.freq, other.freq) {
            Some(f1.cmp(&f2))
        } else {
            None
        }
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq.cmp(&other.freq)
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
            let n = Node::new_node(*k, *v);
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
