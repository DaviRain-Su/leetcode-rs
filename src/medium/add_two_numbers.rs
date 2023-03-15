use crate::utils::list::ListNode;

//主要优化点如下：
//
// 将flag重命名为carry，使其意义更加明确。
// 将三个while循环合并为一个，可以减少代码重复。只要l1或l2或carry中至少有一个不为空，就会继续循环。
// 使用if let Some(node)结构简化链表节点的访问，减少.unwrap()的使用。
// 更新value时直接累加carry，并在每次循环之后重新计算carry的值。
// 删除一些不必要的else子句。
// 这个优化后的版本更加简洁，可读性更强，并减少了重复代码。
pub fn add_two_numbers(
    l1: Option<Box<ListNode<i32>>>,
    l2: Option<Box<ListNode<i32>>>,
) -> Option<Box<ListNode<i32>>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut carry = 0;
    let mut sum = Some(Box::new(ListNode::new(0)));
    let mut current = &mut sum;

    while l1.is_some() || l2.is_some() || carry > 0 {
        let mut value = carry;
        if let Some(node) = l1 {
            value += node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            value += node.val;
            l2 = node.next;
        }

        carry = value / 10;
        current.as_mut().unwrap().next = Some(Box::new(ListNode::new(value % 10)));
        current = &mut current.as_mut().unwrap().next;
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
