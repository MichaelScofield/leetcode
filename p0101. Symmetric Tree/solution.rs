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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root == None {
            return true;
        }
        let mut current_depth_nodes = vec![root];
        loop {
            let mut i = 0;
            let mut j = current_depth_nodes.len() - 1;
            while i < j {
                let node_i_val =
                    if let Some(node_i) = &current_depth_nodes[i] {
                        Some(node_i.borrow().val)
                    } else {
                        None
                    };
                let node_j_val =
                    if let Some(node_j) = &current_depth_nodes[j] {
                        Some(node_j.borrow().val)
                    } else {
                        None
                    };
                if node_i_val == node_j_val {
                    i += 1;
                    j -= 1;
                } else {
                    return false;
                }
            }

            let mut next_depth_nodes = Vec::new();
            let mut is_last_depth = true;
            for node in current_depth_nodes {
                if let Some(node) = node {
                    let node = node.borrow();
                    if let Some(ref left_node) = node.left {
                        next_depth_nodes.push(Some(left_node.clone()));
                        is_last_depth = false;
                    } else {
                        next_depth_nodes.push(None);
                    }

                    if let Some(ref right_node) = node.right {
                        next_depth_nodes.push(Some(right_node.clone()));
                        is_last_depth = false;
                    } else {
                        next_depth_nodes.push(None);
                    }
                }
            }
            if is_last_depth {
                break;
            }
            current_depth_nodes = next_depth_nodes;
        }
        true
    }
}
