use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn pseudo_palindromic_paths (root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // let mut counts = ;
        Self::dfs(root, [0; 9])
    }

    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut s: [i32; 9]) -> i32 {
        // Depth first search
        if let Some(r) = root {
            let node = r.borrow();
            s[(node.val - 1) as usize] += 1;

            // Do calculation if leaf node
            if node.left == None && node.right == None {
                if s.iter().filter(|x| *x % 2 == 1).count() <= 1 { 
                    return 1; 
                }
                else {
                    return 0;
                }
            }

            // Otherwise...
            Self::dfs(node.left.clone(), s) + Self::dfs(node.right.clone(), s.clone())
        }
        else {
            0
        }
    }
}
