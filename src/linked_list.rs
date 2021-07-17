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

pub struct ListNode<T> {
    pub val: T,
    pub next: LinkedList<T>,
}
