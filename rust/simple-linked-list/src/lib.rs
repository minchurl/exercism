use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Self { data, next }
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut current_node = &self.head;
        let mut size = 0;

        while let Some(x) = current_node {
            current_node = &x.next;
            size += 1;
        }
        size
    }

    pub fn push(&mut self, _element: T) {
        let back_node = self.head.take();
        self.head = Some(Box::new(Node::new(_element, back_node)));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            None
        }
        else {
            let back_node = self.head.take().unwrap();
            self.head = back_node.next;

            Some(back_node.data)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.head.is_none() {
            return None;
        }
        else {
            Some(&self.head.as_ref().unwrap().data)
        }
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut ret = Self::new();
        while let Some(x) = self.pop() {
            ret.push(x);
        }

        ret
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut ret: SimpleLinkedList<T> = SimpleLinkedList::new();

        for item in _iter {
            ret.push(item);
        }

        ret
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut ret: Vec<T> = Vec::new();
        let mut rev_linked_list = _linked_list.rev();

        while let Some(x) = rev_linked_list.pop() {
            ret.push(x);
        }

        ret
    }
}
