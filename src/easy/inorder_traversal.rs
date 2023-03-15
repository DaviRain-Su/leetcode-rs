use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// 这个代码使用递归的方式实现二叉树的中序遍历。我们首先检查根节点是否存在，如果存在，则递归地遍历其左子树，
/// 然后将根节点的值加入到结果向量中，最后递归地遍历其右子树。在遍历左右子树时，我们也会进行类似的递归遍历。
///
///注意，由于 root 是一个可选项，我们需要使用 as_ref() 方法将其转换为一个引用，以便在递归调用 inorder_traversal_helper 函数时传递引用。
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> Vec<i32> {
    let mut result = Vec::new();
    inorder_traversal_helper(root.as_ref(), &mut result);
    result
}

fn inorder_traversal_helper(root: Option<&Rc<RefCell<TreeNode<i32>>>>, result: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        inorder_traversal_helper(node.left.as_ref(), result);
        result.push(node.val);
        inorder_traversal_helper(node.right.as_ref(), result);
    }
}

/// 这个代码使用一个栈来模拟递归遍历。我们首先将根节点压入栈中，然后遍历其左子树，将左子节点压入栈中。
/// 在遍历完左子树后，我们弹出栈顶元素并将其加入结果向量中，然后遍历其右子树，将右子节点压入栈中。
/// 这样不断地遍历栈中的元素，直到所有节点都被遍历完为止。
///
/// 注意，由于 Rust 中的 Rc 和 RefCell 会导致一些借用检查的问题，我们需要在 inorder_traversal 函数中对根节点进行一个拷贝，以便在非递归遍历时可以对其进行多次借用。
pub fn inorder_traversal_v2(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut stack = Vec::new();
    let mut curr = root;

    while curr.is_some() || !stack.is_empty() {
        while let Some(node) = curr {
            stack.push(node.clone());
            curr = node.borrow().left.clone();
        }

        let node = stack.pop().unwrap();
        let node = node.borrow();
        result.push(node.val);

        curr = node.right.clone();
    }

    result
}

#[test]
fn test_inorder_traversal() {
    use crate::utils::tree::build_tree;

    let tree = build_tree(&[Some(1), None, Some(2), Some(3)]);
    assert_eq!(inorder_traversal(tree), vec![1, 3, 2]);
    let tree = build_tree::<i32>(&[]);
    assert_eq!(inorder_traversal(tree), vec![]);
    let tree = build_tree(&[Some(1)]);
    assert_eq!(inorder_traversal(tree), vec![1]);

    let tree = build_tree(&[Some(1), None, Some(2), Some(3)]);
    assert_eq!(inorder_traversal_v2(tree), vec![1, 3, 2]);
    let tree = build_tree::<i32>(&[]);
    assert_eq!(inorder_traversal_v2(tree), vec![]);
    let tree = build_tree(&[Some(1)]);
    assert_eq!(inorder_traversal_v2(tree), vec![1]);
}
