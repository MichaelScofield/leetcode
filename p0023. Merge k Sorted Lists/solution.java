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
    public ListNode mergeKLists(ListNode[] lists) {
        ListNode head = null;
        ListNode curr = null;
        List<ListNode> ps = Arrays.stream(lists).filter(Objects::nonNull).collect(Collectors.toList());
        while (!ps.isEmpty()) {
            int minVal = Integer.MAX_VALUE;
            int minListIndex = -1;
            for (int i = 0; i < ps.size(); i++) {
                ListNode p = ps.get(i);
                if (p.val <= minVal) {
                    minVal = p.val;
                    minListIndex = i;
                }
            }
            ListNode node = new ListNode(minVal);
            if (head == null) {
                head = curr = node;
            } else {
                curr.next = node;
                curr = node;
            }
            ListNode pNext = ps.get(minListIndex).next;
            if (pNext == null) {
                ps.remove(minListIndex);
            } else {
                ps.set(minListIndex, pNext);
            }
        }
        return head;
    }
}
