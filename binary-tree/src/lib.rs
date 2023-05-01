use std::cmp::Ordering;

#[derive(Clone, Hash, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Bst<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> Default for Bst<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<T> Node<T>
where
    T: Ord + Clone + std::fmt::Debug,
{
    fn leaf(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn find(&self, predicate: T) -> Option<Box<Node<T>>> {
        Self::find_node(Some(Box::new(self.to_owned())), predicate)
    }

    fn find_node(node: Option<Box<Node<T>>>, predicate: T) -> Option<Box<Node<T>>> {
        if let Some(node) = node {
            return match predicate.cmp(&node.value) {
                Ordering::Less => {
                    println!("is less? {:?}", node.left);
                    Self::find_node(node.left, predicate)
                }
                Ordering::Equal => {
                    println!("is EQUAL? {:?}", node);
                    Some(node)
                }
                Ordering::Greater => {
                    println!("is greater? {:?}", node.right);
                    Self::find_node(node.right, predicate)
                }
            };
        }
        None
    }
}

impl<T> Bst<T>
where
    T: Ord + std::fmt::Debug + Clone,
{
    pub fn from_value(value: T) -> Self {
        let root = Box::new(Node::leaf(value));
        Self { root: Some(root) }
    }

    pub fn insert(&mut self, value: T) {
        let new_node = Box::new(Node::leaf(value));
        Self::push_node(new_node, &mut self.root);
    }

    fn push_node(mut new_node: Box<Node<T>>, curr_node: &mut Option<Box<Node<T>>>) {
        if let Some(node) = curr_node {
            match &new_node.value.cmp(&node.value) {
                Ordering::Less | Ordering::Equal => {
                    // swap curr_node.left = new_node.value
                    if let Some(left) = node.left.clone() {
                        if left.value < new_node.value {
                            node.left = Some(new_node);
                            new_node = left;
                        }
                    }
                    Self::push_node(new_node, &mut node.left);
                }
                Ordering::Greater => {
                    // swap curr_node.right = new_node.value
                    if let Some(right) = node.right.clone() {
                        if right.value > new_node.value {
                            node.right = Some(new_node);
                            new_node = right;
                        }
                    }
                    Self::push_node(new_node, &mut node.right);
                }
            }
        } else {
            *curr_node = Some(new_node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tree = Bst::from_value(3);
        tree.insert(5);
        tree.insert(4);
        tree.insert(7);
        tree.insert(6);
        tree.insert(1);
        tree.insert(2);
        println!("{:#?}", tree);
    }

    #[test]
    fn can_find() {
        let mut tree = Bst::from_value(3);
        tree.insert(5);
        tree.insert(4);
        let found = tree.root.unwrap().find(5);
        // let found2 = tree.find(5);

        println!("-- found {:#?}", found);
    }

    #[test]
    fn can_traverse() {
        let mut tree = Bst::from_value(3);
        tree.insert(5);
        tree.insert(4);
        tree.insert(2);
    }
}
