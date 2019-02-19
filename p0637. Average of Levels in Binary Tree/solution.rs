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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        use std::collections::VecDeque;

        let mut level_averages = Vec::new();
        let mut current_level = 0;
        let mut level_sum: f64 = 0.0;
        let mut level_nodes = 0;

        let mut queue = VecDeque::new();
        queue.push_back((0, root.unwrap()));
        loop {
            let node = queue.pop_front();
            if node == None {
                level_averages.push(level_sum / (level_nodes as f64));
                break;
            }

            let (level, node_ref) = node.unwrap();
            let node = node_ref.borrow();
            if current_level != level {
                level_averages.push(level_sum / (level_nodes as f64));
                current_level = level;
                level_sum = node.val as f64;
                level_nodes = 1;
            } else {
                level_sum += node.val as f64;
                level_nodes += 1;
            }

            if let Some(left_node) = &node.left {
                queue.push_back((level + 1, left_node.clone()));
            }
            if let Some(right_node) = &node.right {
                queue.push_back((level + 1, right_node.clone()));
            }
        }
        level_averages
    }
}
