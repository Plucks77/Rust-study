use std::iter::FromIterator;
use std::mem;
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        return self.head.is_none();
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        return count;
    }

    pub fn push(&mut self, element: T) {
        let new_node = Some(Box::new(Node {
            data: element,
            next: mem::replace(&mut self.head, None), // Same as self.head.take()
        }));
        self.head = new_node;
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = mem::replace(&mut self.head, None);
        match head {// Same as self.head.take()
            Some(head) => {
                self.head = head.next;
                return Some(head.data);
            }
            None => {
                return None;
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        let head = self.head.as_ref();
        if let Some(head) = head {
            return Some(&head.data);
        }
        return None;
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        match self.head {
            None => SimpleLinkedList::new(),
            Some(head) => {
                let mut list = SimpleLinkedList::new();
                let mut current = Some(head);
                while let Some(node) = current {
                    list.push(node.data);
                    current = node.next;
                }
                return list;
            }
        }
    }
}

impl<T: std::fmt::Debug> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut linked_list = SimpleLinkedList::new();
        for item in iter {
            linked_list.push(item);
        }
        return linked_list;
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::new();
        let mut current = linked_list.head;

        while let Some(node) = current {
            vec.push(node.data);
            current = node.next;
        }

        vec.reverse();
        return vec;
    }
}

fn main() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);
    list.push(6);
    list.push(7);
    list.push(8);

    let mut v = Vec::new();
    let mut s = SimpleLinkedList::new();
    for i in 1..4 {
        v.push(i);
        s.push(i);
    }
    let s_as_vec: Vec<i32> = s.into();

    println!("x: {:?}", s_as_vec);
}

#[test]
fn test_new_list_is_empty() {
    let list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    assert_eq!(list.len(), 0, "list's length must be 0");
}
#[test]
fn test_push_increments_length() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    assert_eq!(list.len(), 1, "list's length must be 1");
    list.push(2);
    assert_eq!(list.len(), 2, "list's length must be 2");
}
#[test]
fn test_pop_decrements_length() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    list.pop();
    assert_eq!(list.len(), 1, "list's length must be 1");
    list.pop();
    assert_eq!(list.len(), 0, "list's length must be 0");
}
#[test]
fn test_is_empty() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    assert!(list.is_empty(), "List wasn't empty on creation");
    for inserts in 0..100 {
        for i in 0..inserts {
            list.push(i);
            assert!(
                !list.is_empty(),
                "List was empty after having inserted {}/{} elements",
                i,
                inserts
            );
        }
        for i in 0..inserts {
            assert!(
                !list.is_empty(),
                "List was empty before removing {}/{} elements",
                i,
                inserts
            );
            list.pop();
        }
        assert!(
            list.is_empty(),
            "List wasn't empty after having removed {} elements",
            inserts
        );
    }
}
#[test]
fn test_pop_returns_head_element_and_removes_it() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    assert_eq!(list.pop(), Some(2), "Element must be 2");
    assert_eq!(list.pop(), Some(1), "Element must be 1");
    assert_eq!(list.pop(), None, "No element should be contained in list");
}
#[test]
fn test_peek_returns_reference_to_head_element_but_does_not_remove_it() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    assert_eq!(list.peek(), None, "No element should be contained in list");
    list.push(2);
    assert_eq!(list.peek(), Some(&2), "Element must be 2");
    assert_eq!(list.peek(), Some(&2), "Element must be still 2");
    list.push(3);
    assert_eq!(list.peek(), Some(&3), "Head element is now 3");
    assert_eq!(list.pop(), Some(3), "Element must be 3");
    assert_eq!(list.peek(), Some(&2), "Head element is now 2");
    assert_eq!(list.pop(), Some(2), "Element must be 2");
    assert_eq!(list.peek(), None, "No element should be contained in list");
}
#[test]
fn test_from_slice() {
    let mut array = vec!["1", "2", "3", "4"];
    let mut list: SimpleLinkedList<_> = array.drain(..).collect();
    assert_eq!(list.pop(), Some("4"));
    assert_eq!(list.pop(), Some("3"));
    assert_eq!(list.pop(), Some("2"));
    assert_eq!(list.pop(), Some("1"));
}
#[test]
fn test_reverse() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut rev_list = list.rev();
    assert_eq!(rev_list.pop(), Some(1));
    assert_eq!(rev_list.pop(), Some(2));
    assert_eq!(rev_list.pop(), Some(3));
    assert_eq!(rev_list.pop(), None);
}
#[test]
fn test_into_vector() {
    let mut v = Vec::new();
    let mut s = SimpleLinkedList::new();
    for i in 1..4 {
        v.push(i);
        s.push(i);
    }
    let s_as_vec: Vec<i32> = s.into();
    assert_eq!(v, s_as_vec);
}
