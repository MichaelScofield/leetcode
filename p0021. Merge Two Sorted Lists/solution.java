/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
class Solution {
    public ListNode mergeTwoLists(ListNode l1, ListNode l2) {
        if (l1 == null) {
            return l2;
        }
        if (l2 == null) {
            return l1;
        }
        ListNode head = null;
        ListNode last = null;
        while (l1 != null || l2 != null) {
            int v;
            if (l1 == null) {
                v = l2.val;
                l2 = l2.next;
            } else if (l2 == null) {
                v = l1.val;
                l1 = l1.next;
            } else {
                int v1 = l1.val;
                int v2 = l2.val;
                if (v1 < v2) {
                    v = v1;
                    l1 = l1.next;
                } else {
                    v = v2;
                    l2 = l2.next;
                }
            }
            ListNode current = new ListNode(v);
            if (head == null) {
                head = current;
            } else {
                last.next = current;
            }
            last = current;
        }
        return head;
    }
}
