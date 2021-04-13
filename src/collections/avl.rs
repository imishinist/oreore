use crate::collections::avl::Balance::{Balanced, LeftLean, RightLean};
use std::cmp::{max, Ordering};

#[derive(Debug, PartialEq)]
struct Node<T> {
    key: Option<T>,
    height: usize,

    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[derive(Debug, PartialEq)]
enum Balance {
    LeftLean(usize),
    RightLean(usize),
    Balanced,
}

impl From<Balance> for i32 {
    fn from(balance: Balance) -> Self {
        match balance {
            Balance::LeftLean(n) => n as i32,
            Balance::RightLean(n) => -(n as i32),
            Balance::Balanced => 0,
        }
    }
}

impl From<i32> for Balance {
    fn from(i: i32) -> Self {
        match i.cmp(&0) {
            Ordering::Equal => Balanced,
            Ordering::Greater => LeftLean(i as usize),
            Ordering::Less => RightLean(-i as usize),
        }
    }
}

impl<T> Node<T> {
    fn new(k: T) -> Self {
        Node {
            key: Some(k),
            height: 1,
            left: None,
            right: None,
        }
    }

    fn key(&self) -> &T {
        self.key.as_ref().unwrap()
    }

    fn left_key(&self) -> Option<&T> {
        self.left.as_ref().map(|f| f.key())
    }

    fn right_key(&self) -> Option<&T> {
        self.right.as_ref().map(|f| f.key())
    }

    fn incr_height(&mut self) {
        self.height = max_height(&self) + 1;
    }
}

#[inline]
fn height<T>(node: &Option<Box<Node<T>>>) -> usize {
    match node {
        Some(n) => n.height,
        None => 0,
    }
}

#[inline]
fn max_height<T>(node: &Node<T>) -> usize {
    max(height(&node.left), height(&node.right))
}

#[inline]
fn get_balance<T>(node: &Node<T>) -> Balance {
    (height(&node.left) as i32 - height(&node.right) as i32).into()
}

fn right_rotate<T>(mut y: Box<Node<T>>) -> Box<Node<T>> {
    let mut x = y.left.unwrap();
    y.left = x.right.take();
    y.incr_height();

    x.right = Some(y);
    x.incr_height();

    x
}

fn left_rotate<T>(mut x: Box<Node<T>>) -> Box<Node<T>> {
    let mut y = x.right.unwrap();
    x.right = y.left.take();
    x.height = max_height(&x) + 1;

    y.left = Some(x);
    y.height = max_height(&y) + 1;

    y
}

fn insert_with_node<T>(node: &mut Option<Box<Node<T>>>, key: T) -> Box<Node<T>>
where
    T: PartialOrd + Clone,
{
    if node.is_none() {
        return Box::new(Node::new(key));
    }
    let mut node = node.take().unwrap();

    match key.partial_cmp(node.key()) {
        None => return node,
        Some(Ordering::Less) => node.left = Some(insert_with_node(&mut node.left, key.clone())),
        Some(Ordering::Greater) => {
            node.right = Some(insert_with_node(&mut node.right, key.clone()))
        }
        Some(Ordering::Equal) => return node,
    }
    node.incr_height();

    match get_balance(&node) {
        // left left
        Balance::LeftLean(n) if n > 1 && &key < node.left_key().unwrap() => right_rotate(node),
        // left right
        Balance::LeftLean(n) if n > 1 && &key > node.right_key().unwrap() => {
            node.left = Some(left_rotate(node.left.unwrap()));
            right_rotate(node)
        }
        // right right
        Balance::RightLean(n) if n > 1 && &key > node.right_key().unwrap() => left_rotate(node),
        // right left
        Balance::RightLean(n) if n > 1 && &key < node.right_key().unwrap() => {
            node.right = Some(right_rotate(node.right.unwrap()));
            left_rotate(node)
        }
        _ => node,
    }
}

#[allow(unused_variables)]
fn min_value_node<T>(node: &Node<T>) -> Box<Node<T>> {
    todo!()
}

#[allow(unused_variables)]
fn delete_node<T>(node: &Option<Box<Node<T>>>, key: T) -> Box<Node<T>> {
    todo!()
}

#[derive(Debug, Default, PartialEq)]
pub struct Tree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: PartialOrd + Clone> Tree<T> {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        self.root = Some(insert_with_node(&mut self.root, value));
    }

    pub fn delete(&mut self, value: T) {
        self.root = Some(delete_node(&self.root, value));
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::avl::Balance::{Balanced, LeftLean, RightLean};
    use crate::collections::avl::{Balance, Node, Tree};

    macro_rules! bin_tree {
        ( key: $key:expr, height: $height:expr, left: $left:expr, right: $right:expr $(,)? ) => {
            Node {
                key: Some($key),
                height: $height,
                left: Some(Box::new($left)),
                right: Some(Box::new($right)),
            }
        };
        ( key: $key:expr, height: $height:expr, right: $right:expr $(,)? ) => {
            Node {
                key: Some($key),
                height: $height,
                left: None,
                right: Some(Box::new($right)),
            }
        };
        ( key: $key:expr, height: $height:expr, left: $left:expr $(,)? ) => {
            Node {
                key: Some($key),
                height: $height,
                left: Some(Box::new($left)),
                right: None,
            }
        };
        (key: $key:expr, height: $height:expr $(,)? ) => {
            Node {
                key: Some($key),
                height: $height,
                left: None,
                right: None,
            }
        };
    }

    #[test]
    fn i32_to_balance_test() {
        assert_eq!(Balance::from(0), Balanced);
        assert_eq!(Balance::from(1), LeftLean(1));
        assert_eq!(Balance::from(-1), RightLean(1));
    }

    #[test]
    fn balance_to_i32_test() {
        assert_eq!(i32::from(Balanced), 0);
        assert_eq!(i32::from(LeftLean(1)), 1);
        assert_eq!(i32::from(RightLean(1)), -1);
    }

    #[test]
    fn test_insert() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(20);
        tree.insert(30);
        tree.insert(40);
        tree.insert(50);
        tree.insert(25);

        let root = Some(Box::new(bin_tree! {
            key: 30,
            height: 3,
            left: bin_tree! {
                key: 20,
                height: 2,
                left: bin_tree! {
                    key: 10,
                    height: 1,
                },
                right: bin_tree! {
                    key: 25,
                    height: 1,
                }
            },
            right: bin_tree! {
                key: 40,
                height: 2,
                right: bin_tree! {
                    key: 50,
                    height: 1,
                }
            }
        }));
        let expected = Tree { root };

        assert_eq!(tree, expected);
    }
}
