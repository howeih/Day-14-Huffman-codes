use std::collections::HashMap;

#[derive(Debug)]
enum Data {
    Count(i32),
    Leaf(String, i32),
}

struct Tree {
    node_id: i32,
    tree: HashMap<i32, Node>,
}

#[derive(Debug)]
struct Node {
    node_id: i32,
    data: Data,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            node_id: 0,
            tree: HashMap::<i32, Node>::new(),
        }
    }

    fn get_node_id(&mut self) -> i32 {
        let node_id = self.node_id;
        self.node_id += 1;
        node_id
    }

    fn insert_node(&mut self, data: Data) {
        let node_id = self.get_node_id();
        let node = Node {
            node_id: node_id,
            left: None,
            right: None,
            data: data,
        };
        self.tree.insert(node_id, node);
    }

    fn get_tree_len(&self) -> usize {
        self.tree.len()
    }

    fn get_node_count(&self, data: &Data) -> i32 {
        *match data {
            Data::Count(c) => c,
            Data::Leaf(_ch, c) => c,
        }
    }

    fn get_and_remove_node(&mut self, node_id: i32) -> Node {
        self.tree.remove(&node_id).unwrap()
    }

    fn merge(&mut self, l_node: Node, r_node: Node) -> i32 {
        let node_id = self.get_node_id();
        let merge_count = self.get_node_count(&l_node.data) + self.get_node_count(&r_node.data);
        let merge_node = Node {
            node_id: node_id,
            left: Some(Box::new(l_node)),
            right: Some(Box::new(r_node)),
            data: Data::Count(merge_count),
        };
        self.tree.insert(node_id, merge_node);
        node_id
    }

    fn find_min(&mut self) -> Node {
        let mut min_node_id: i32 = -1;
        let mut min_count = std::i32::MAX;
        for (n, d) in &self.tree {
            match d.data {
                Data::Count(count) => {
                    if count < min_count {
                        min_count = count;
                        min_node_id = *n;
                    }
                }
                Data::Leaf(ref _ch, count) => {
                    if count < min_count {
                        min_count = count;
                        min_node_id = *n;
                    }
                }
            }
        }
        self.tree.remove(&min_node_id).unwrap()
    }
}

fn print_code(node: &Option<Box<Node>>, code: &str) {
    match &node {
        Some(n) => match &n.data {
            Data::Count(_) => {
                let code_str = [code, "0"].join("");
                print_code(&n.left, &code_str);
                let code_str = [code, "1"].join("");
                print_code(&n.right, &code_str);
            }
            Data::Leaf(ch, _) => {
                println!("{}: {}", ch, code);
            }
        },
        None => {}
    }
}

fn huffman_codes(text: &str) {
    let mut freq = HashMap::<char, i32>::new();
    let mut tree = Tree::new();
    let mut merge_id = -1;
    for i in text.chars() {
        let counter = freq.entry(i).or_insert(0);
        *counter += 1;
    }
    for (ch, count) in freq {
        tree.insert_node(Data::Leaf(ch.to_string(), count));
    }
    while tree.get_tree_len() > 1 {
        let l_node_id = tree.find_min();
        let r_node_id = tree.find_min();
        merge_id = tree.merge(l_node_id, r_node_id);
    }
    let root = tree.get_and_remove_node(merge_id);
    let code = "";
    print_code(&Some(Box::new(root)), code);
}

fn main() {
    huffman_codes("astala vista tasta");
}
