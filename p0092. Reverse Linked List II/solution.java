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
    public ListNode reverseBetween(ListNode head, int m, int n) {
        if (head == null) {
            return null;
        }
        assert m >= 1 && n >= m;
        ListNode prev = null;
        ListNode curr = head;
        for (int i = 1; i < m; i++) {
            prev = curr;
            curr = curr.next;
        }
        ListNode reverse = reverse(curr, n - m + 1);
        if (prev == null) {
            return reverse;
        }
        prev.next = reverse;
        return head;
    }

    ListNode reverse(ListNode node, int len) {
        assert node != null;
        ListNode prev = null;
        ListNode curr = node;
        int i = 0;
        while (curr != null && i++ < len) {
            ListNode next = curr.next;
            curr.next = prev;
            prev = curr;
            curr = next;
        }
        node.next = curr;
        return prev;
    }
}
