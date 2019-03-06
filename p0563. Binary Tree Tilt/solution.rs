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
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root == None {
            return 0;
        }
        let mut tilt = 0;

        fn sum(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let node = node.unwrap();
            let node = node.borrow();
            let left_tree_sum =
                if let Some(ref left_node) = node.left {
                    sum(Some(left_node.clone()))
                } else {
                    0
                };
            let right_tree_sum =
                if let Some(ref right_node) = node.right {
                    sum(Some(right_node.clone()))
                } else {
                    0
                };
            left_tree_sum + node.val + right_tree_sum
        }

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

            let mut left_tree_sum = 0;
            if let Some(ref left_node) = node.left {
                left_tree_sum = sum(Some(left_node.clone()));
                queue.push_back(left_node.clone());
            }

            let mut right_tree_sum = 0;
            if let Some(ref right_node) = node.right {
                right_tree_sum = sum(Some(right_node.clone()));
                queue.push_back(right_node.clone());
            }

            tilt += (left_tree_sum - right_tree_sum).abs();
        }
        tilt
    }
}
