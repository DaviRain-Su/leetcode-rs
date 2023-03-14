// https://dawchihliou.github.io/articles/binary-tree-insertion-in-rust
// https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370
// https://codereview.stackexchange.com/questions/133209/binary-tree-implementation-in-rust
// https://reecem.medium.com/writing-a-binary-search-tree-in-rust-b0cf180f3f16
// https://dev-notes.eu/2020/03/Binary-Search-in-Rust/
// https://shane-o.dev/blog/binary-search-rust
// https://medium.com/@abrar.nitk/rust-binary-search-tree-149969b87f72
// https://stackoverflow.com/questions/57633089/searching-a-string-into-vecstring-in-rust

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, node: Option<Rc<RefCell<TreeNode<T>>>>) -> Self {
        self.left = node;
        self
    }

    pub fn right(mut self, node: Option<Rc<RefCell<TreeNode<T>>>>) -> Self {
        self.right = node;
        self
    }

    pub fn from(new_values: &[T]) -> Self {
        todo!()
    }
}

pub fn sum_numbers<T>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> T {
    todo!()
}

#[test]
fn test_sum_numbers() -> anyhow::Result<()> {
    // let tree = Rc::new(RefCell::new(TreeNode::from(vec![1, 2, 3])));
    // let tree = build_tree(&vec![1, 2, 3]);
    // println!("tree is {:#?}", tree);
    // assert_eq!(sum_numbers(tree), 25);
    // let tree = TreeNode::build_tree(&vec![4, 9, 0, 5, 1]);
    // println!("tree is {:#?}", tree);
    // let tree = Rc::new(RefCell::new(TreeNode::from(vec![4, 9, 0, 5, 1])));
    // assert_eq!(sum_numbers(tree), 1026);

    Ok(())
}
