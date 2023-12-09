use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => {
                vec![]
            },
            Some(r) => {
                let node = r.borrow();
                let mut left = Self::inorder_traversal(node.left.clone());
                let mut right = Self::inorder_traversal(node.right.clone());
                
                left.push(node.val);
                left.append(&mut right);

                left
            }
        }
    }
}
