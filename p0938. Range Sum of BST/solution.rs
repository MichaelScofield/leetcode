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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        let mut sum = 0;
        if let Some(node) = root {
            let node = node.borrow();
            if node.val >= l && node.val <= r {
                sum += node.val;
            }

            if node.val > l {
                let left_node = &node.left;
                if let Some(left_node_ref) = left_node {
                    sum += Solution::range_sum_bst(Some(left_node_ref.clone()), l, r);
                }
            }

            if node.val < r {
                let right_node = &node.right;
                if let Some(right_node_ref) = right_node {
                    sum += Solution::range_sum_bst(Some(right_node_ref.clone()), l, r);
                }
            }
        }
        sum
    }
}
