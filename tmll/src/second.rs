use std::mem;

pub struct List<T> {
    head: Link<T>,
}

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn IterMut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

type Link<T> = Option<Box<Node<T>>>;

// simple tuple struct wrapper around our list type
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn new() -> Self {
        List { head: Link::None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Link::Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::None);

        // consume each node starting with the first
        while let Link::Some(mut boxed_node) = cur_link {
            // swing forward to the next node
            cur_link = mem::replace(&mut boxed_node.next, Link::None);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.pop(), None);
        assert_eq!(list.peek(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);
        assert_eq!(list.peek(), Some(&5));
        list.peek_mut().map(|val| {
            *val = 42;
        });
        assert_eq!(list.pop(), Some(42));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));

        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn iter() {
        let list: List<i32> = List::new();
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn try_map() {
        let maybe_string = Some(String::from("I'm here!"));
        let maybe_len = maybe_string.map(|v| v.len());
        println!("{}", maybe_len.unwrap());
    }
}
