use std::cmp::max;

#[derive(Debug, PartialEq)]
struct Node<T> {
    key: Option<T>,
    height: usize,

    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

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
        if i == 0 {
            Balance::Balanced
        } else if i > 0 {
            Balance::LeftLean(i as usize)
        } else {
            Balance::RightLean(-i as usize)
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
}

#[inline]
fn height<T>(node: &Option<Box<Node<T>>>) -> usize {
    match node {
        Some(n) => n.height,
        None => 0,
    }
}

#[inline]
fn max_height<T>(node: &Box<Node<T>>) -> usize {
    max(height(&node.left), height(&node.right))
}

#[inline]
fn get_balance<T>(node: &Option<Box<Node<T>>>) -> Balance {
    match node {
        Some(n) => {
            let b = height(&n.left) as i32 - height(&n.right) as i32;
            b.into()
        }
        None => Balance::Balanced,
    }
}

fn right_rotate<T>(node: &Box<Node<T>>) -> Box<Node<T>> {
    todo!()
}

fn left_rotate<T>(node: &Box<Node<T>>) -> Box<Node<T>> {
    todo!()
}

fn insert_with_node<T>(node: &Option<Box<Node<T>>>, key: T) -> Box<Node<T>> {
    todo!()
}

fn min_value_node<T>(node: &Box<Node<T>>) -> Box<Node<T>> {
    todo!()
}

fn delete_node<T>(node: &Option<Box<Node<T>>>, key: T) -> Box<Node<T>> {
    todo!()
}

pub struct Tree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        self.root = Some(insert_with_node(&self.root, value));
    }

    pub fn delete(&mut self, value: T) {
        self.root = Some(delete_node(&self.root, value));
    }
}
