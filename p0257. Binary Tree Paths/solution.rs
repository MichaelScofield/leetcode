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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut all_paths = Vec::new();
        Solution::dfs(root, "", &mut all_paths);
        all_paths
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, last_path: &str, all_paths: &mut Vec<String>) {
        if let Some(node) = node {
            let node = node.borrow();
            let val = node.val.to_string();
            let current_path = if last_path.len() == 0 {
                val
            } else {
                last_path.clone().to_owned() + "->" + &val
            };
            if node.left.is_none() && node.right.is_none() {
                all_paths.push(current_path);
            } else {
                Solution::dfs(node.left.clone(), &current_path, all_paths);
                Solution::dfs(node.right.clone(), &current_path, all_paths);
            }
        }
    }
}
