use std::cmp::Ordering;
#[allow(unused, unused_variables, dead_code)]
use std::collections::VecDeque;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn from_value(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Bst<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> Default for Bst<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<T> Bst<T>
where
    T: Ord + Sized + PartialOrd + Clone,
{
    fn new() -> Self {
        Default::default()
    }

    fn from_value(value: T) -> Self {
        let root = Box::new(Node::from_value(value));
        Self { root: Some(root) }
    }

    fn insert(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            left: None,
            right: None,
        });
        self.push_node(new_node, self.root);
    }

    fn push_node(&mut self, new_node: Box<Node<T>>, curr_node: &mut Option<Box<Node<T>>>) {
        if let Some(node) = curr_node {
            match node.value.cmp(&new_node.value) {
                Ordering::Less | Ordering::Equal => {
                    // if the lesser value is larger than the current one
                    // means that the tree is unbalanced
                    if node.value > new_node.value {
                        let node_holder = node.clone();
                        // swap curr node with the new node
                        node.value = new_node.value.clone();
                        // restart iteration with the swapped node at the root
                        let root_as_node = Box::new(Node {
                            value: self.root.clone().unwrap().value,
                            left: self.root.clone().unwrap().left,
                            right: self.root.clone().unwrap().right,
                        });
                        self.push_node(node_holder, &mut Some(root_as_node));
                    } else {
                        self.push_node(new_node, &mut node.left);
                    }
                }
                Ordering::Greater => self.push_node(new_node, &mut node.right),
            }
        } else {
            curr_node.insert(new_node);
        }
    }
}

fn main() {
    let mut tree = Bst::from_value(3);
    tree.insert(4);
    tree.insert(1);
    tree.insert(12);
    tree.insert(2);
    println!("{:#?}", tree);
}
