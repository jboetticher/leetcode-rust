use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            if r.val >= low && r.val <= high { 
                return r.val + 
                    Self::range_sum_bst(r.right.clone(), low , high) + 
                    Self::range_sum_bst(r.left.clone(), low, high)
            }
            else if r.val < low { return Self::range_sum_bst(r.right.clone(), low , high) }
            else if r.val > high { return Self::range_sum_bst(r.left.clone(), low, high) }
        }
        0
    }
}
