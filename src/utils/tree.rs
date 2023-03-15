use std::cell::RefCell;
use std::rc::Rc;

type Node<T> = Option<Rc<RefCell<TreeNode<T>>>>;
/// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Node<T>,
    pub right: Node<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tee<T> {
    pub root: Option<TreeNode<T>>,
}

impl<T: Ord> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from(value: &[Option<T>]) -> Self {
        todo!()
    }

    #[inline]
    pub fn left(mut self, value: Node<T>) -> Self {
        self.left = value;
        self
    }

    #[inline]
    pub fn right(mut self, value: Node<T>) -> Self {
        self.right = value;
        self
    }
}

fn insert<T: Ord>(node: &mut Node<T>, value: T) {
    if node.is_none() {
        *node = Some(Rc::new(RefCell::new(TreeNode::new(value))));
    } else if value < node.as_ref().unwrap().borrow().val {
        insert(&mut node.as_mut().unwrap().borrow_mut().left, value);
    } else {
        insert(&mut node.as_mut().unwrap().borrow_mut().right, value);
    }
}

/// 这个代码会首先构造根节点，然后一层一层地构造二叉树。
/// 在每一层中，我们使用一个数组来存储该层的所有节点，并逐个遍历它们。
/// 对于每个节点，我们检查它的左右子节点是否存在，如果存在，则构造新的节点并将其赋给对应的子节点。
pub fn build_tree<T: Clone>(nums: &[Option<T>]) -> Node<T> {
    if nums.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode {
        val: nums[0].as_ref().unwrap().clone(),
        left: None,
        right: None,
    }));

    let mut level_nodes = vec![root.clone()];
    let mut index = 1;

    while !level_nodes.is_empty() {
        let mut next_level_nodes = Vec::new();
        for node in level_nodes {
            if index >= nums.len() {
                break;
            }
            if let Some(left_val) = &nums[index] {
                let left_node = Rc::new(RefCell::new(TreeNode {
                    val: left_val.clone(),
                    left: None,
                    right: None,
                }));
                node.borrow_mut().left = Some(left_node.clone());
                next_level_nodes.push(left_node);
            }
            index += 1;

            if index >= nums.len() {
                break;
            }
            if let Some(right_val) = &nums[index] {
                let right_node = Rc::new(RefCell::new(TreeNode {
                    val: right_val.clone(),
                    left: None,
                    right: None,
                }));
                node.borrow_mut().right = Some(right_node.clone());
                next_level_nodes.push(right_node);
            }
            index += 1;
        }
        level_nodes = next_level_nodes;
    }

    Some(root)
}

pub fn build_binary_tree<T: Clone>(nums: &[T]) -> Node<T> {
    fn build_tree_helper<T: Clone>(nums: &[T], index: usize) -> Node<T> {
        if index >= nums.len() {
            return None;
        }

        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[index].clone(),
            left: build_tree_helper(nums, 2 * index + 1),
            right: build_tree_helper(nums, 2 * index + 2),
        })))
    }

    build_tree_helper(nums, 0)
}

#[test]
fn test_tree_insert() {
    let mut root: Node<i32> = None;
    insert(&mut root, 3);
    insert(&mut root, 1);
    insert(&mut root, 5);
    println!("{:#?}", root);

    let array = vec![Some(1), None, Some(2), Some(3)];
    let result = build_tree(&array);
    println!("{:#?}", result);

    // let array = vec![1, 2, 3, 4, 5, 6];
    // let binary_tree = build_binary_tree(&array);
    // println!("binary tree = {binary_tree:#?}");
}
