//Huffman tree
use std::{cmp::Ordering, cmp::Ordering::Equal, collections::HashMap, fmt::Display, fs};

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
        matches!((&self.left, &self.right), (None, None))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl PartialOrd for Node {
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
        match self.val {
            None => write!(f, "Non-leaf Node: {}", self.freq),
            Some(val) => write!(f, "Leaf Node--val:{} freq:{}", val, self.freq),
        }
    }
}

pub struct NodeArray {
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

        Self { nodes }
    }

    pub fn build_huffman_tree(&self) {}

    fn select_children(&self, n1: &<'_> Node, n2: &Node) -> (&Node, &Node) {
        if self.nodes.len() <= 0 {
            return (n1, n2);
        }
        let t1 = &self.nodes.last().unwrap();
        let t2 = &self.nodes.last().unwrap();

        (t1, t2)
    }
}

impl Display for NodeArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert_ne!(self.nodes.len(), 0);
        for node in &self.nodes {
            writeln!(f, "{}", node)?;
        }

        Ok(())
    }
}
