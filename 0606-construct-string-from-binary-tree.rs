use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
      match root {
          None => "".to_string(),
          Some(node) => {
            let d = node.borrow();
            let left = Self::tree2str(d.left.clone());
            let right = Self::tree2str(d.right.clone());

            if left == "" && right == "" {
              d.val.to_string()
            }
            else if right == "" {
              format!("{}({})", d.val, left)
            }
            else {
              format!("{}({})({})", d.val, left, right)
            }
          }
      }
    }
}
