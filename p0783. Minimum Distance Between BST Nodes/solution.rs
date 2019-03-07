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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root == None {
            return 0;
        }
        let mut values = Vec::new();
        fn traverse(node: Rc<RefCell<TreeNode>>, values: &mut Vec<i32>) {
            let node = node.borrow();
            if let Some(ref left_node) = node.left {
                traverse(left_node.clone(), values);
            }
            values.push(node.val);
            if let Some(ref right_node) = node.right {
                traverse(right_node.clone(), values);
            }
        }
        traverse(root.unwrap(), &mut values);
        let mut min_diff = std::i32::MAX;
        for i in 1..values.len() {
            let diff = values[i] - values[i - 1];
            if diff < min_diff {
                min_diff = diff;
            }
        }
        min_diff
    }
}
