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
    pub fn merge_trees(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::VecDeque;

        if t1.is_none() {
            return t2;
        }
        if t2.is_none() {
            return t1;
        }
        let t1_clone = t1.clone();

        let mut queue =
            VecDeque::<(Rc<RefCell<TreeNode>>, Rc<RefCell<TreeNode>>)>::new();
        queue.push_back((t1.unwrap(), t2.unwrap()));
        loop {
            let t = queue.pop_front();
            if t.is_none() {
                break;
            }

            let (e0, e1) = t.unwrap();
            let mut n1 = e0.borrow_mut();
            let mut n2 = e1.borrow_mut();
            n1.val += n2.val;

            if n2.left.is_some() {
                let rc = Rc::clone(&n2.left.as_mut().unwrap());
                if n1.left.is_some() {
                    queue.push_back((Rc::clone(&n1.left.as_mut().unwrap()), rc));
                } else {
                    n1.left = Some(rc);
                }
            }

            if n2.right.is_some() {
                let rc = Rc::clone(&n2.right.as_mut().unwrap());
                if n1.right.is_some() {
                    queue.push_back((Rc::clone(&n1.right.as_mut().unwrap()), rc));
                } else {
                    n1.right = Some(rc);
                }
            }
        }
        return t1_clone;
    }
}
