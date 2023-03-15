use crate::utils::list::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode<i32>>>,
    l2: Option<Box<ListNode<i32>>>,
) -> Option<Box<ListNode<i32>>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut flag = false;
    let mut sum: Option<Box<ListNode<i32>>> = Some(Box::new(ListNode::<i32>::default()));
    let mut current = &mut sum;
    while l1.is_some() && l2.is_some() {
        let one = if flag { 1 } else { 0 };
        let s = l1.as_ref().unwrap().val + l2.as_ref().unwrap().val + one;
        if s >= 10 {
            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(s % 10)));
            flag = true;
        } else {
            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(s)));
            flag = false;
        }
        current = &mut current.as_mut().unwrap().next;
        l1 = l1.unwrap().next;
        l2 = l2.unwrap().next;
    }

    while l1.is_some() {
        let one = if flag { 1 } else { 0 };
        let l1_v = l1.as_ref().unwrap().val + one;
        if l1_v >= 10 {
            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(l1_v % 10)));
            flag = true;
        } else {
            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(l1_v)));
            flag = false;
        }
        current = &mut current.as_mut().unwrap().next;
        l1 = l1.unwrap().next;
    }
    while l2.is_some() {
        let one = if flag { 1 } else { 0 };
        let l2_v = l2.as_ref().unwrap().val + one;
        if l2_v >= 10 {
            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(l2_v % 10)));
            flag = true;
        } else {
            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(l2_v)));
            flag = false;
        }
        current = &mut current.as_mut().unwrap().next;
        l2 = l2.unwrap().next;
    }
    if flag {
        current.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
    }
    sum.unwrap().next
}

#[test]
fn test_add_two_numbers() {
    use crate::utils::list::build_list;
    let l1 = build_list(vec![2, 4, 3]);
    let l2 = build_list(vec![5, 6, 4]);
    let l3 = build_list(vec![7, 0, 8]);
    assert_eq!(l3, add_two_numbers(l1, l2));

    let l1 = build_list(vec![0]);
    let l2 = build_list(vec![0]);
    let l3 = build_list(vec![0]);
    assert_eq!(l3, add_two_numbers(l1, l2));

    let l1 = build_list(vec![9, 9, 9, 9, 9, 9, 9]);
    let l2 = build_list(vec![9, 9, 9, 9]);
    let l3 = build_list(vec![8, 9, 9, 9, 0, 0, 0, 1]);
    assert_eq!(l3, add_two_numbers(l1, l2));

    let l1 = build_list(vec![2, 4, 9]);
    let l2 = build_list(vec![5, 6, 4, 9]);
    let l3 = build_list(vec![7, 0, 4, 0, 1]);
    assert_eq!(l3, add_two_numbers(l1, l2));

    let l1 = build_list(vec![3, 1, 0, 0, 1, 9, 0, 1, 6, 1]);
    let l2 = build_list(vec![5, 5, 8, 6, 2, 5, 8, 2, 6, 1]);
    let l3 = build_list(vec![8, 6, 8, 6, 3, 4, 9, 3, 2, 3]);
    assert_eq!(l3, add_two_numbers(l1, l2));
}
