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
    T: Ord + Sized + PartialOrd + Clone + std::fmt::Debug,
{
    fn new() -> Self {
        Default::default()
    }

    fn from_value(value: T) -> Self {
        let root = Box::new(Node::from_value(value));
        Self { root: Some(root) }
    }

    fn insert(&mut self, value: T) {
        println!("insert - self {:?}", self.root);
        println!("insert - value {:?}", value);
        let new_node = Box::new(Node::from_value(value));
        Self::push_node(new_node, &mut self.root);
    }

    fn push_node(mut new_node: Box<Node<T>>, curr_node: &mut Option<Box<Node<T>>>) {
        if let Some(node) = curr_node {
            match &new_node.value.cmp(&node.value) {
                Ordering::Less | Ordering::Equal => {
                    // swap curr_node.left = new_node.value
                    if let Some(left) = node.left.clone() {
                        if left.value < new_node.value {
                            node.left.insert(new_node.clone());
                            new_node = left;
                        }
                    }
                    Self::push_node(new_node, &mut node.left);
                }
                Ordering::Greater => {
                    println!("if - new_node {:?}", new_node);
                    println!("if - node.right {:?}", node.right);
                    // swap curr_node.right = new_node.value
                    if let Some(right) = node.right.clone() {
                        if right.value > new_node.value {
                            node.right.insert(new_node.clone());
                            new_node = right;
                        }
                    }
                    Self::push_node(new_node, &mut node.right);
                }
            }
        } else {
            // here, curr_node is None, because it is
            // a new Node that will be added to the tree.
            println!("else - new_node {:?}", new_node.value);
            println!("else - curr_node {:?}", curr_node);
            // insert new_node into `right` or `left`
            // of the curr_node. Making it a Some.
            curr_node.insert(new_node);
            println!("-----");
        }
    }
}

fn main() {
    let mut tree = Bst::from_value(3);
    tree.insert(5);
    tree.insert(4);
    tree.insert(7);
    tree.insert(6);
    tree.insert(1);
    tree.insert(2);
    println!("{:#?}", tree);
}
