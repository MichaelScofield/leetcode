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
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut node_x = None;
        let mut node_y = None;

        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back((-1, 0, root.unwrap())); // (parent_val, depth, node)
        loop {
            let node = queue.pop_front();
            if node == None {
                break;
            }
            let (parent_val, depth, node) = node.unwrap();

            let node = node.borrow();
            if node.val == x {
                node_x = Some((parent_val, depth));
            } else if node.val == y {
                node_y = Some((parent_val, depth));
            }
            if node_x != None && node_y != None {
                let (x_parent_val, x_depth) = node_x.unwrap();
                let (y_parent_val, y_depth) = node_y.unwrap();
                return x_parent_val != y_parent_val && x_depth == y_depth;
            }

            if let Some(ref left_node) = node.left {
                queue.push_back((node.val, depth + 1, left_node.clone()));
            }
            if let Some(ref right_node) = node.right {
                queue.push_back((node.val, depth + 1, right_node.clone()));
            }
        }
        false
    }
}
