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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut values = Vec::new();

        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());
        loop {
            let node = queue.pop_front();
            if node == None {
                break;
            }

            let node = node.unwrap();
            let node = node.borrow();
            values.push(node.val);

            if let Some(ref left_node) = node.left {
                queue.push_back(left_node.clone());
            }
            if let Some(ref right_node) = node.right {
                queue.push_back(right_node.clone());
            }
        }

        values.sort();

        let mut min_diff = i32::max_value();
        for i in 0..values.len() - 1 {
            let diff = (values[i] - values[i + 1]).abs();
            if diff < min_diff {
                min_diff = diff;
            }
        }
        min_diff
    }
}
