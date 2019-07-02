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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let mut v = Vec::new();
            Solution::traverse(root.clone(), 0, &mut v);
            v.iter().sum()
        } else {
            0
        }
    }

    fn traverse(node: Rc<RefCell<TreeNode>>, n: i32, v: &mut Vec<i32>) {
        let node = node.borrow();
        let val = node.val;
        let n = (n << 1) | val;
        if node.left.is_none() && node.right.is_none() {
            v.push(n);
        } else {
            if let Some(ref left_node) = node.left {
                Solution::traverse(left_node.clone(), n, v);
            }
            if let Some(ref right_node) = node.right {
                Solution::traverse(right_node.clone(), n, v);
            }
        }
    }
}
