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
    public ListNode deleteDuplicates(ListNode head) {
        if (head == null) {
            return null;
        }
        ListNode newHead = null;
        ListNode last = head, curr = head;
        while (curr != null && curr.next != null) {
            if (curr.val == curr.next.val) {
                ListNode nextNotDuplicate = findNextNotDuplicate(curr);
                last.next = curr = nextNotDuplicate;
            } else {
                if (newHead == null) {
                    newHead = curr;
                }
                last = curr;
                curr = curr.next;
            }
        }
        return newHead == null ? curr : newHead;
    }

    ListNode findNextNotDuplicate(ListNode head) {
        ListNode curr = head;
        while (curr != null) {
            if (curr.next == null) {
                return null;
            }
            if (curr.val != curr.next.val) {
                return curr.next;
            }
            curr = curr.next;
        }
        return null;
    }
}
