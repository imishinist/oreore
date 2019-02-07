use std::marker::PhantomData;
use std::ptr::NonNull;


#[derive(Debug)]
struct AVLTree<T> {
    root: Option<Box<AVLNode<T>>>,
    change: bool,
    lmax_value: T,
}

#[derive(Debug)]
struct AVLNode<T> {
    height: usize,
    value: T,
    lst: Option<Box<AVLNode<T>>>,
    rst: Option<Box<AVLNode<T>>>,
    phantom: PhantomData<T>,
}

impl<T> AVLNode<T> {
    fn new(height: usize, value: T) -> Self {
        AVLNode {
            height, value, lst: None, rst: None, phantom: PhantomData::<T>
        }
    }
}

fn get<T>(node: Option<&Box<AVLNode<T>>>, value: &T) -> Option<&T> {
    match node {
        None => None,
        Some(box n) if n < value => get(node.lst, &value),
        Some(box n) if n > value => get(node.rst, &value),
        Some(box n) => return Some(&n.value),
    }
    None
}

impl<T> AVLTree<T> where T: PartialOrd {
    pub fn get(&self, other: &T) -> Option<&T> {
        get(self.root.as_ref(), &other)
    }
}


fn height<T>(t: Option<&Box<AVLNode<T>>>) -> usize {
    match t {
        None => 0,
        Some(node) => node.height,
    }
}

fn bias<T>(t: Option<&Box<AVLNode<T>>>) -> i32 {
    match t {
        None => 0,
        Some(node) => (height(node.lst.as_ref()) - height(node.rst.as_ref())) as i32,
    }
}

fn mod_height<T>(t: &mut Box<AVLNode<T>>) {
    let lst_height = height(t.lst.as_ref());
    let rst_height = height(t.rst.as_ref());
    t.height = 1 + std::cmp::max(lst_height, rst_height);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height() {
        assert_eq!(1, height(Some(&box AVLNode::new(1, 1))));
        assert_eq!(0, height::<usize>(None));
    }

    #[test]
    fn test_bias() {
        assert_eq!(0, bias(Some(&box AVLNode::new(1, 1))));
    }

    #[test]
    fn test_mod_height() {
        let lst = Some(box AVLNode::new(1, 1));
        let rst = Some(box AVLNode::new(1, 1));
        let mut root = box AVLNode { height: 1, value: 2, lst, rst, phantom: PhantomData::<i32>};

        assert_eq!(0, bias(Some(&root)));
        assert_eq!(1, height(Some(&root)));
        mod_height(&mut root);
        assert_eq!(2, height(Some(&root)));
    }
}
