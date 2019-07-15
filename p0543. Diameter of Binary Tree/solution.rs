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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut diameter = 1;
        Solution::max_depth(root.clone(), &mut diameter);
        diameter - 1
    }

    fn max_depth(root: Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let max_depth_left = Solution::max_depth(node.left.clone(), max_diameter);
            let max_depth_right = Solution::max_depth(node.right.clone(), max_diameter);
            *max_diameter = std::cmp::max(*max_diameter, max_depth_left + max_depth_right + 1);
            1 + std::cmp::max(max_depth_left, max_depth_right)
        } else {
            0
        }
    }
}
