use std::marker::PhantomData;
use std::cmp::PartialOrd;

#[derive(Debug)]
struct Tree<T> {
    right: Option<Box<Tree<T>>>,
    left: Option<Box<Tree<T>>>,
    phantom: PhantomData<T>,
    data: Option<T>
}

impl<T: PartialEq> PartialEq for Tree<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.right.is_none() ^ other.right.is_none() ||
            self.left.is_none() ^ other.left.is_none() {
            return false;
        }
        self.data.eq(&other.data) && self.right.eq(&other.right) && self.left.eq(&other.left)
    }
}

impl<T> Tree<T> where T: PartialOrd {
    pub fn new(value: T) -> Self {
        Tree {
            right: None,
            left: None,
            phantom: PhantomData::<T>,
            data: Some(value)
        }
    }

    pub fn new_root() -> Self {
        Tree {
            right: None,
            left: None,
            phantom: PhantomData::<T>,
            data: None
        }
    }

    pub fn add(&mut self, value: T) {
        match &self.data {
            None => {},
            Some(ref n) if n < &value => {
                match self.left {
                    None => self.left = Some(Box::new(Tree::new(value))),
                    Some(ref mut left_tree) => left_tree.add(value)
                }
            },
            Some(_n) => {
                match self.right {
                    None => self.right = Some(Box::new(Tree::new(value))),
                    Some(ref mut right_tree) => right_tree.add(value)
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_root_returns_empty_tree() {
        let tree: Tree<i32> = Tree::new_root();
        assert!(tree.eq(&Tree::<i32>{right: None, left: None, phantom: PhantomData::<i32>, data: None}));
    }

    #[test]
    fn new_returns_tree() {
        let tree: Tree<i32> = Tree::new(10);
        assert!(tree.eq(&Tree::<i32>{right: None, left: None, phantom: PhantomData::<i32>, data: Some(10)}));

        let tree = Tree::new("hoge".to_string());
        assert!(tree.eq(&Tree::<String>{right: None, left: None, phantom: PhantomData::<String>, data: Some("hoge".to_string())}));
    }

    #[test]
    fn eq_test() {
        let phantom = PhantomData::<i32>;
        assert_eq!(Tree::<i32>{right: None, left: None, phantom, data: Some(10)},
                   Tree::<i32>{right: None, left: None, phantom, data: Some(10)});

        assert_ne!(
            Tree::<i32>{
                right: Some(Box::new(Tree::<i32>{right: None, left: None, phantom, data: Some(15)})),
                left: None,
                phantom,
                data: Some(10)},
            Tree::<i32>{
                right: None,
                left: None,
                phantom,
                data: Some(10)});

        assert_ne!(
            Tree::<i32>{
                right: Some(Box::new(Tree::<i32>{right: None, left: None, phantom, data: Some(15)})),
                left: None,
                phantom,
                data: Some(10)},
            Tree::<i32>{
                right: Some(Box::new(Tree::<i32>{right: None, left: None, phantom, data: Some(12)})),
                left: None,
                phantom,
                data: Some(10)});

        assert_eq!(
            Tree::<i32>{
                right: Some(Box::new(Tree::<i32>{right: None, left: None, phantom, data: Some(15)})),
                left: None,
                phantom,
                data: Some(10)},
            Tree::<i32>{
                right: Some(Box::new(Tree::<i32>{right: None, left: None, phantom, data: Some(15)})),
                left: None,
                phantom,
                data: Some(10)});
    }

}
