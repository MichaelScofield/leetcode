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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut level_nodes: Vec<Vec<i32>> = Vec::new();

        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back((0, root.unwrap()));
        loop {
            let node = queue.pop_front();
            if let Some((level, node_ref)) = node {
                let node = node_ref.borrow();
                if level_nodes.len() < level + 1 {
                    level_nodes.push(vec![node.val]);
                } else {
                    level_nodes[level].push(node.val);
                }

                if let Some(left_node) = &node.left {
                    queue.push_back((level + 1, left_node.clone()));
                }
                if let Some(right_node) = &node.right {
                    queue.push_back((level + 1, right_node.clone()));
                }
            } else {
                break;
            }
        }
        level_nodes.reverse();
        level_nodes
    }
}
