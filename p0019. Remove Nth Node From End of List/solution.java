/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
    public ListNode removeNthFromEnd(ListNode head, int n) {
        if (head == null) {
            return null;
        }
        if (n <= 0) {
            return head;
        }
        ListNode fast = head, slow = head;
        for (int i = 0; i < n - 1; i++) {
            if (fast.next != null) {
                fast = fast.next;
            } else {
                return head;
            }
        }
        ListNode prev = null;
        while (fast.next != null) {
            fast = fast.next;
            prev = slow;
            slow = slow.next;
        }
        if (prev == null) {
            return head.next;
        } else {
            prev.next = slow.next;
            return head;
        }
    }
}
