use std::iter::FromIterator;

pub struct LinkedList<T>(pub Option<Box<ListNode<T>>>);

impl<T> LinkedList<T> {
    pub fn new(val: T) -> Self {
        Self(Some(Box::new(ListNode {
            val,
            next: Self(None),
        })))
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        vec.into_iter().collect()
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut head = Self(None);
        let mut curr = &mut head;
        for val in iter {
            *curr = LinkedList::new(val);
            curr = &mut curr.0.as_mut().unwrap().next;
        }
        head
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIterator(self)
    }
}

pub struct LinkedListIterator<T>(LinkedList<T>);

impl<T> Iterator for LinkedListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.0.0.take()?;
        *self = x.next.into_iter();
        Some(x.val)
    }
}

pub struct ListNode<T> {
    pub val: T,
    pub next: LinkedList<T>,
}
