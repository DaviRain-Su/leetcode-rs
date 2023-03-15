// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode { next: None, val }
    }
}

pub fn build_list<T: Default>(values: Vec<T>) -> Option<Box<ListNode<T>>> {
    let mut head = Some(Box::new(ListNode::default()));
    let mut current = &mut head;
    for value in values.into_iter() {
        current.as_mut().unwrap().next = Some(Box::new(ListNode::new(value)));
        current = &mut current.as_mut().unwrap().next;
    }
    head.unwrap().next
}

impl<T: Default> From<Vec<T>> for ListNode<T> {
    fn from(value: Vec<T>) -> Self {
        *build_list(value).unwrap()
    }
}

#[test]
fn test_build_list() {
    let values = vec![1, 2, 3, 4, 5];
    let list = build_list(values);
    println!("{:#?}", list);
}
