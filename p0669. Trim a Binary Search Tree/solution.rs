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
    pub fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node_ref) = root {
            let mut node = node_ref.borrow_mut();
            if node.val < l {
                return Solution::trim_bst(node.right.clone(), l, r);
            }
            if node.val > r {
                return Solution::trim_bst(node.left.clone(), l, r);
            }
            node.left = Solution::trim_bst(node.left.clone(), l, r);
            node.right = Solution::trim_bst(node.right.clone(), l, r);
            Some(node_ref.clone())
        } else {
            None
        }
    }
}
