/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) {
 *         val = x;
 *         next = null;
 *     }
 * }
 */
public class Solution {
    public ListNode getIntersectionNode(ListNode headA, ListNode headB) {
        if (headA == null || headB == null) {
            return null;
        }
        int l1 = len(headA), l2 = len(headB);
        if (l1 > l2) {
            headA = forward(headA, l1 - l2);
        } else {
            headB = forward(headB, l2 - l1);
        }
        while (headA != null && headB != null) {
            if (headA == headB) {
                return headA;
            }
            headA = headA.next;
            headB = headB.next;
        }
        return null;
    }

    int len(ListNode head) {
        int l = 0;
        while (head != null) {
            l += 1;
            head = head.next;
        }
        return l;
    }

    ListNode forward(ListNode head, int n) {
        while (n-- > 0) {
            head = head.next;
        }
        return head;
    }
}
