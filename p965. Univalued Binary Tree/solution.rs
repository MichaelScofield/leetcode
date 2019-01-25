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
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        use std::collections::VecDeque;

        let root = root.unwrap();
        let expect_val = root.borrow().val;
        let mut is_unival = true;
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));
        loop {
            let node = queue.pop_front();
            match node {
                None => break,
                Some(node) => {
                    let node = node.borrow();
                    if node.val != expect_val {
                        is_unival = false;
                        break;
                    } else {
                        if let Some(ref left_node) = node.left {
                            queue.push_back(Rc::clone(left_node));
                        }
                        if let Some(ref right_node) = node.right {
                            queue.push_back(Rc::clone(right_node));
                        }
                    }
                }
            }
        }
        is_unival
    }
}
