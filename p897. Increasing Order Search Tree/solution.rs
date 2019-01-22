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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut values = Vec::new();
        Solution::in_order_traversal(root, &mut values);
        if values.len() == 0 {
            return None;
        }

        let root: Rc<RefCell<TreeNode>> =
            Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut curr_node = Rc::clone(&root);
        for i in 1..values.len() {
            let node =
                Rc::new(RefCell::new(TreeNode::new(values[i].unwrap())));
            curr_node.borrow_mut().right = Some(Rc::clone(&node));
            curr_node = node;
        }
        Some(Rc::clone(&root))
    }

    fn in_order_traversal(node: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<Option<i32>>) {
        let node = node.unwrap();
        let node = node.borrow();

        if let Some(ref left_node) = node.left {
            Solution::in_order_traversal(Some(Rc::clone(left_node)), values);
        }

        values.push(Some(node.val));

        if let Some(ref right_node) = node.right {
            Solution::in_order_traversal(Some(Rc::clone(right_node)), values);
        }
    }
}
