// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut traversal: Vec<i32> = vec![];
        match root {
            None => traversal,
            Some(node) => {
                let node = node.borrow();
                traversal.push(node.val);
                traversal.append(&mut Solution::preorder_traversal(node.left.clone()));
                traversal.append(&mut Solution::preorder_traversal(node.right.clone()));
                traversal
            }
        }
    }
}
