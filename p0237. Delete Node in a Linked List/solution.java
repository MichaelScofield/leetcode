/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
public class Solution {
    public void deleteNode(ListNode node) {
        ListNode next = node.next;
        ListNode nextNext = next.next;
        node.next = nextNext;
        node.val = next.val;
    }
}
