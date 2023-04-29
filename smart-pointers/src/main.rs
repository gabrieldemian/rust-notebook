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
        println!("insert - value {:?}", value);
        println!("insert - self {:?}", self);
        let new_node = Box::new(Node::from_value(value));
        Self::push_node(new_node, &mut self.root);
    }

    fn push_node(new_node: Box<Node<T>>, curr_node: &mut Option<Box<Node<T>>>) {
        if let Some(node) = curr_node {
            match &new_node.value.cmp(&node.value) {
                Ordering::Less | Ordering::Equal => {
                    //
                    Self::push_node(new_node, &mut node.left);
                }
                Ordering::Greater => {
                    println!(
                        "if - new_node {:?} to node.right {:?}",
                        new_node, node.right
                    );
                    Self::push_node(new_node, &mut node.right);
                }
            }
        } else {
            // here, curr_node is None, because it is
            // a new Node that will be added to the tree.
            println!("else - new_node {:?}", new_node);
            println!("else - curr_node {:?}", curr_node);
            // insert new_node into `right` or `left`
            // of the curr_node. Making it a Some.
            curr_node.insert(new_node);
        }
    }
}

fn main() {
    let mut tree = Bst::from_value(3);
    tree.insert(5);
    tree.insert(4);
    println!("{:#?}", tree);
}
