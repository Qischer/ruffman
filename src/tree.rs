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

impl Node {
    pub fn new_node(val: Option<char>, freq: usize) -> Self {
        Self {
            left: None,
            right: None,

            val,
            freq,
        }
    }

    pub fn new_parent(left: Node, right: Node) -> Self {
        let freq = &left.freq + &right.freq;
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

pub struct Huffman {
    root: Option<Box<Node>>,
}

impl Huffman {
    pub fn new_from_file(src: &str) -> Self {
        //file IO
        let binding = fs::read_to_string(src).unwrap();
        let content = binding.chars();

        let mut heap = BinaryHeap::new();
        let mut freq = HashMap::new();

        for c in content {
            freq.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }

        for (k, v) in freq.iter() {
            let n = Node::new_node(Some(*k), *v);
            heap.push(Reverse(n));
        }

        while heap.len() > 1 {
            let n1 = heap.pop().unwrap().0;
            let n2 = heap.pop().unwrap().0;

            let p = Node::new_parent(n1, n2);

            heap.push(Reverse(p));
        }

        let root = heap.pop().unwrap().0;

        Self {
            root: Some(Box::new(root)),
        }
    }

    pub fn translate(&self) {
        let code = String::new();
        match &self.root {
            None => return,
            Some(node) => Self::translate_helper(node, &code),
        }
    }

    fn translate_helper(node: &Box<Node>, code: &str) {
        if node.is_leaf() {
            println!("{} : {}", node.val.unwrap(), code);
        }

        if let Some(left) = &node.left {
            let left_code = String::from(code.to_owned() + "0");
            Self::translate_helper(&left, &left_code);
        }

        if let Some(right) = &node.right {
            let right_code = String::from(code.to_owned() + "1");
            Self::translate_helper(&right, &right_code);
        }
    }
}

impl Display for Huffman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.root {
            None => write!(f, "Huffman tree is Empty"),
            Some(a) => write!(f, "{}", a),
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

        let mut nodes = vec![];

        for c in content {
            freq.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }

        for (k, v) in freq.iter() {
            let n = Node::new_node(Some(*k), *v);
            nodes.push(n);
        }

        Self { nodes }
    }

    pub fn build_huffman_tree<'a>(&'a mut self) {}
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
