mod tree;

#[cfg(test)]
mod test {
    use super::*;
    use std::cmp::Ordering::*;

    #[test]
    fn compare_nodes_eq() {
        let n1 = tree::Node::new_node(Some('a'), 1);
        let n2 = tree::Node::new_node(Some('a'), 1);

        let tmp = n1.cmp(&n2);
        assert_eq!(tmp, Equal);
    }

    #[test]
    fn compare_nodes_freq() {
        let n1 = tree::Node::new_node(Some('a'), 1);
        let n2 = tree::Node::new_node(Some('a'), 2);

        let tmp = n1.cmp(&n2);
        assert_eq!(tmp, Less);
    }

    #[test]
    fn compare_nodes_val() {
        {
            let n1 = tree::Node::new_node(Some('a'), 1);
            let n2 = tree::Node::new_node(Some('b'), 1);

            let tmp = n1.cmp(&n2);
            assert_eq!(tmp, Less);
        }
        {
            let n1 = tree::Node::new_node(Some('B'), 1);
            let n2 = tree::Node::new_node(Some('b'), 1);

            let tmp = n1.cmp(&n2);
            assert_eq!(tmp, Less);
        }
    }
}
