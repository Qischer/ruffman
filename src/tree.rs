//Huffman tree
use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,

    val: char,
    freq: usize,
}

impl Node {
    pub fn new_node(val: char, freq: usize) -> Self {
        Self {
            left: None,
            right: None,

            val,
            freq,
        }
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
            println!("key: {k} - value: {v}");
            let n = Node::new_node(*k, *v);
            nodes.push(n);
        }

        Self {
            n_char: nodes.len(),
            capacity: 256,
            nodes,
        }
    }
}
