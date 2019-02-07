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
            right: Option::None,
            left: Option::None,
            phantom: PhantomData::<T>,
            data: Option::Some(value)
        }
    }

    pub fn new_root() -> Self {
        Tree {
            right: Option::None,
            left: Option::None,
            phantom: PhantomData::<T>,
            data: Option::None
        }
    }

    pub fn add(&mut self, value: T) {
        match &self.data {
            Option::None => {},
            Option::Some(ref n) if n < &value => {
                match self.left {
                    Option::None => self.left = Option::Some(box Tree::new(value)),
                    Option::Some(ref mut left_tree) => left_tree.add(value)
                }
            },
            Option::Some(_n) => {
                match self.right {
                    Option::None => self.right = Option::Some(box Tree::new(value)),
                    Option::Some(ref mut right_tree) => right_tree.add(value)
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
        assert!(tree.eq(&Tree::<i32>{right: Option::None, left: Option::None, phantom: PhantomData::<i32>, data: Option::None}));
    }

    #[test]
    fn new_returns_tree() {
        let tree: Tree<i32> = Tree::new(10);
        assert!(tree.eq(&Tree::<i32>{right: Option::None, left: Option::None, phantom: PhantomData::<i32>, data: Option::Some(10)}));

        let tree = Tree::new("hoge".to_string());
        assert!(tree.eq(&Tree::<String>{right: Option::None, left: Option::None, phantom: PhantomData::<String>, data: Option::Some("hoge".to_string())}));
    }

    #[test]
    fn eq_test() {
        assert_eq!(Tree::<i32>{right: Option::None, left: Option::None, phantom: PhantomData::<i32>, data: Option::Some(10)},
                   Tree::<i32>{right: Option::None, left: Option::None, phantom: PhantomData::<i32>, data: Option::Some(10)});

        assert_ne!(
            Tree::<i32>{
                right: Option::Some(Box::new(Tree::<i32>{right: Option::None, left: Option::None, phantom: PhantomData::<i32>, data: Option::Some(15)})),
                left: Option::None,
                phantom: PhantomData::<i32>,
                data: Option::Some(10)},
            Tree::<i32>{
                right: Option::None,
                left: Option::None,
                phantom: PhantomData::<i32>,
                data: Option::Some(10)});

        assert_ne!(
            Tree::<i32>{
                right: Option::Some(Box::new(Tree::<i32>{right: Option::None, left: Option::None, phantom: PhantomData::<i32>, data: Option::Some(15)})),
                left: Option::None,
                phantom: PhantomData::<i32>,
                data: Option::Some(10)},
            Tree::<i32>{
                right: Option::Some(Box::new(Tree::<i32>{right: Option::None, left: Option::None, phantom: PhantomData::<i32>, data: Option::Some(12)})),
                left: Option::None,
                phantom: PhantomData::<i32>,
                data: Option::Some(10)});

        assert_eq!(
            Tree::<i32>{
                right: Option::Some(Box::new(Tree::<i32>{right: Option::None, left: Option::None, phantom: PhantomData::<i32>, data: Option::Some(15)})),
                left: Option::None,
                phantom: PhantomData::<i32>,
                data: Option::Some(10)},
            Tree::<i32>{
                right: Option::Some(Box::new(Tree::<i32>{right: Option::None, left: Option::None, phantom: PhantomData::<i32>, data: Option::Some(15)})),
                left: Option::None,
                phantom: PhantomData::<i32>,
                data: Option::Some(10)});
    }

}
