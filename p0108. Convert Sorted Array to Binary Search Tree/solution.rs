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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        match nums.len() {
            0 => None,
            _ => Solution::create_bst(nums.as_slice())
        }
    }

    fn create_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let l = nums.len();
        let mid = l / 2;
        let mut node = TreeNode::new(nums[mid]);
        if mid > 0 {
            node.left = Solution::create_bst(&nums[0..mid]);
        }
        if mid + 1 < l {
            node.right = Solution::create_bst(&nums[(mid + 1)..]);
        }
        Some(Rc::new(RefCell::new(node)))
    }
}
