use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let leaves_1 = Self::depth_leaf_to_vec(root1);
        let leaves_2 = Self::depth_leaf_to_vec(root2);
        leaves_1.len() == leaves_2.len() && leaves_1.iter().zip(leaves_2.iter()).all(|x| x.0 == x.1)        
    }

    // Iterative preorder DFS
    pub fn depth_leaf_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut deque: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut ans = vec![];
        if let Some(root) = root {
            deque.push_back(root);
        }
        while !deque.is_empty() {
            if let Some(node) = deque.pop_front() {
                let node = node.borrow();

                // It is only a leaf if it has zero children
                if node.left == None && node.right == None {
                    ans.push(node.val);
                }
                else {
                    if let Some(right) = node.right.clone() {
                        deque.push_front(right);
                    }
                    if let Some(left) = node.left.clone() {
                        deque.push_front(left);
                    }
                }
            }
        }
        ans
    }
}
