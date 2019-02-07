use std::marker::PhantomData;
use std::ptr::NonNull;
use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    phantom_data: PhantomData<T>,
    length: usize
}

#[derive(Debug)]
struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    element: T,
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.element.eq(&other.element)
    }

    fn ne(&self, other: &Self) -> bool {
        self.element.ne(&other.element)
    }
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            next: None,
            prev: None,
            element: value
        }
    }

    fn into_element(self: Box<Self>) -> T {
        self.element
    }
}

impl<T> LinkedList<T> {

    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            phantom_data: PhantomData,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = self.head;
            node.prev = None;
            let node = Some(Box::into_raw_non_null(node));

            match self.head {
                None => self.tail = node,
                Some(mut head) => head.as_mut().prev = node,
            }
            self.head = node;
            self.length += 1;
        }
    }

    fn push_back_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.prev = self.tail;
            node.next = None;
            let node = Some(Box::into_raw_non_null(node));

            match self.tail {
                None => self.head = node,
                Some(mut tail) => tail.as_mut().next = node,
            }

            self.tail = node;
            self.length += 1;
        }
    }

    pub fn push_front(&mut self, value: T) {
        self.push_front_node(box Node::new(value))
    }

    pub fn push_back(&mut self, value: T) {
        self.push_back_node(box Node::new(value))
    }

    pub fn front(&self) -> Option<&T> {
        unsafe {
            self.head.as_ref().map(|node| &node.as_ref().element )
        }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe {
            self.tail.as_ref().map(|node| &node.as_ref().element )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_empty_list() {
      let list: LinkedList<i32> = LinkedList::new();
      assert!(list.eq(&LinkedList::<i32>{head: Option::None, tail: Option::None, length: 0, phantom_data: PhantomData::<i32>}));

      let mut list: LinkedList<i32> = LinkedList::new();
      assert!(list.eq(&mut LinkedList::<i32>{head: Option::None, tail: Option::None, length: 0, phantom_data: PhantomData::<i32>}));
    }

    #[test]
    fn new_returns_empty_node() {
        let node: Node<i32> = Node::new(1);
        assert_eq!(node, Node{ next: None, prev: None, element: 1});

        let mut node: Node<i32> = Node::new(2);
        assert_eq!(node, Node{ next: None, prev: None, element: 2});
    }

    #[test]
    fn push_and_pop() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(20);
        assert_eq!(list.front(), Some(&20));
        assert_eq!(list.back(), Some(&20));
        assert_eq!(list.len(), 1);
        list.push_front(10);
        assert_eq!(list.front(), Some(&10));
        assert_eq!(list.back(), Some(&20));
        assert_eq!(list.len(), 2);
        list.push_back(30);
        assert_eq!(list.front(), Some(&10));
        assert_eq!(list.back(), Some(&30));
        assert_eq!(list.len(), 3);
    }
}

