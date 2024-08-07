//Huffman tree
use std::{
    cmp::Ordering::Equal,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    fs,
};

pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,

    val: Option<char>,
    freq: usize,
}

impl Node<'_> {
    pub fn new_node() -> Self {
        Self {
            left: None,
            right: None,

            val,
            freq,
        }
    }

    pub fn new_parent<'a>(left: &'a Node<'a>, right: &'a Node<'a>) -> Node<'a> {
        let freq = &left.freq + &right.freq;
        Node {
            left: Some(left),
            right: Some(right),

            val: None,
            freq,
        }
    }

    pub fn is_leaf(&self) -> bool {
        matches!((&self.left, &self.right), (None, None))
    }
}

impl PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl PartialOrd for Node<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.freq.cmp(&other.freq) {
            Equal => {
                if let (Some(v1), Some(v2)) = (self.val, other.val) {
                    Some(v1.cmp(&v2))
                } else {
                    Some(Equal)
                }
            }
            other => Some(other),
        }
    }
}

impl Eq for Node<'_> {}

impl Ord for Node<'_> {
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

impl Display for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.val {
            None => write!(f, "Non-leaf Node: {}", self.freq),
            Some(val) => write!(f, "Leaf Node--val:{} freq:{}", val, self.freq),
        }
    }
}

pub struct NodeArray<'a> {
    nodes: Vec<Node<'a>>,
    parents: Vec<Node<'a>>,
}

impl NodeArray<'_> {
    pub fn new_from_file(src: &str, freq: &mut HashMap<char, usize>) -> Self {
        //file IO
        let binding = fs::read_to_string(src).unwrap();
        let content = binding.chars();

        let (mut nodes, mut parents) = (vec![], vec![]);

        let mut heap = BinaryHeap::new();

        for c in content {
            freq.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }

        for (k, v) in freq.iter() {
            let n = Node::new_node(Some(*k), *v);
            heap.push(Reverse(n));
        }

        let mut _c1: Node;
        let mut _c2: Node;

        let p: &i8;
        let x = 6;

        p = &x;

        println!("{p}");

        Self { nodes, parents }
    }

    pub fn build_huffman_tree<'a>(&'a mut self) {}
}

impl Display for NodeArray<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert_ne!(self.nodes.len(), 0);
        for node in &self.nodes {
            writeln!(f, "{}", node)?;
        }

        Ok(())
    }
}
