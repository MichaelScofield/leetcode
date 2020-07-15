// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sum = Vec::new();
        let mut p1 = l1.clone();
        let mut p2 = l2.clone();
        let mut carry = 0;
        while p1.is_some() || p2.is_some() {
            let v1 = if let Some(n1) = p1 {
                p1 = n1.next;
                n1.val
            } else {
                0
            };
            let v2 = if let Some(n2) = p2 {
                p2 = n2.next;
                n2.val
            } else {
                0
            };
            let mut add = v1 + v2 + carry;
            if add > 9 {
                add -= 10;
                carry = 1;
            } else {
                carry = 0;
            }
            sum.push(add);
        }
        if carry > 0 {
            sum.push(carry);
        }
        let mut head = None;
        for &val in sum.iter().rev() {
            let next = head.take();
            head = Some(Box::new(ListNode { val, next }));
        }
        head
    }
}
