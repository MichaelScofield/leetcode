// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head == None {
            return None;
        }
        let head = head.unwrap();
        let mut fast_p = &head;
        let mut slow_p = &head;
        let mut jump = 0;
        loop {
            match fast_p.next {
                None => break,
                Some(ref next_node) => {
                    jump += 1;
                    if jump % 2 == 0 {
                        slow_p = slow_p.next.as_ref().unwrap();
                    }
                    fast_p = next_node;
                }
            }
        }
        if jump % 2 == 1 {
            slow_p = slow_p.next.as_ref().unwrap();
        }

        // because the method's signature is intended 'move' the original list in and 'move'
        // the middle of it out, which is not possible by Rust (because the ownership rules don't
        // allow it), so we have to create a new list.
        let mut new_head = ListNode::new((&**slow_p).val);
        Solution::copy_values(&mut new_head, slow_p);
        Some(Box::new(new_head))
    }

    fn copy_values(new_head: &mut ListNode, old_head: &ListNode) {
        let mut old_head = old_head;
        let mut new_tail = new_head;
        loop {
            match old_head.next {
                None => break,
                Some(ref next_node) => {
                    let mut new_node = ListNode::new(next_node.val);
                    new_tail.next = Some(Box::new(new_node));

                    // the '{}' around 'new_tail' is used to move (but not borrow) it into a temporary
                    // variable, so we are free to set new value to the original variable, without causing
                    // an "already borrowed" error.
                    // see: https://stackoverflow.com/questions/37986640/cannot-obtain-a-mutable-reference-when-iterating-a-recursive-structure-cannot-b
                    new_tail = { new_tail }.next.as_mut().unwrap();

                    old_head = next_node;
                }
            }
        }
    }
}

// fn main() {
//     let mut n0 = ListNode::new(1);
//     let mut n1 = ListNode::new(2);
//     let mut n2 = ListNode::new(3);
//     let mut n3 = ListNode::new(4);
//     let n4 = ListNode::new(5);
//     n3.next = Some(Box::new(n4));
//     n2.next = Some(Box::new(n3));
//     n1.next = Some(Box::new(n2));
//     n0.next = Some(Box::new(n1));
//     println!("{:?}", Solution::middle_node(Some(Box::new(n0))));

//     let mut n0 = ListNode::new(1);
//     let mut n1 = ListNode::new(2);
//     let mut n2 = ListNode::new(3);
//     let mut n3 = ListNode::new(4);
//     let mut n4 = ListNode::new(5);
//     let n5 = ListNode::new(6);
//     n4.next = Some(Box::new(n5));
//     n3.next = Some(Box::new(n4));
//     n2.next = Some(Box::new(n3));
//     n1.next = Some(Box::new(n2));
//     n0.next = Some(Box::new(n1));
//     println!("{:?}", Solution::middle_node(Some(Box::new(n0))));
// }
