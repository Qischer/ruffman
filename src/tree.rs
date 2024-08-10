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

    pub fn new_parent(left: Node, right: Node) -> Self {
        let freq = &left.freq + &right.freq;
        let val = std::cmp::min(&left.val, &right.val).clone();
        Self {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),

            val,
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
            Equal => Some(self.val.cmp(&other.val)),
            other => Some(other),
        }
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.freq.cmp(&other.freq) {
            Equal => self.val.cmp(&other.val),
            other => other,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Leaf Node--val:{} freq:{}", self.val, self.freq)
    }
}

pub struct Huffman {
    root: Option<Box<Node>>,
    dict: Option<HashMap<char, String>>,
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
            let n = Node::new_node(*k, *v);
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
            dict: None,
        }
    }

    pub fn get_dict(&self) -> Option<HashMap<char, String>> {
        self.dict.clone()
    }

    pub fn translate(&mut self) {
        let code = String::new();
        let mut dict = HashMap::<char, String>::new();

        match &self.root {
            None => return,
            Some(node) => Self::translate_helper(node, &code, &mut dict),
        }

        self.dict = Some(dict)
    }

    fn translate_helper(node: &Box<Node>, code: &str, dict: &mut HashMap<char, String>) {
        if node.is_leaf() {
            dict.insert(node.val, code.to_owned());
        }

        if let Some(left) = &node.left {
            let left_code = String::from(code.to_owned() + "0");
            Self::translate_helper(&left, &left_code, dict);
        }

        if let Some(right) = &node.right {
            let right_code = String::from(code.to_owned() + "1");
            Self::translate_helper(&right, &right_code, dict);
        }
    }
}

impl Display for Huffman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.dict {
            None => write!(f, "Huffman tree is Empty"),
            Some(d) => {
                for (k, v) in d.iter() {
                    write!(f, "{k}: {v}\n")?
                }
                Ok(())
            }
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
            let n = Node::new_node(*k, *v);
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
