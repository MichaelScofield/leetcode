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
    public ListNode sortList(ListNode head) {
        if (head == null) {
            return null;
        }
        return sort(head, null);
    }

    static class Partition {
        final ListNode subListHead;
        final ListNode pivotHead;
        final ListNode subListTail;

        Partition(ListNode subListHead, ListNode pivotHead, ListNode subListTail) {
            this.subListHead = subListHead;
            this.pivotHead = pivotHead;
            this.subListTail = subListTail;
        }

        @Override
        public String toString() {
            return "Partition{" +
                    "subListHead=" + subListHead +
                    ", pivotHead=" + pivotHead +
                    ", subListTail=" + subListTail +
                    '}';
        }
    }

    ListNode sort(ListNode head, ListNode tailNext) {
        if (head == tailNext || head.next == tailNext) {
            return head;
        }
        Partition partition = partition(head, tailNext);
        partition.subListTail.next = tailNext;
        if (partition.subListHead == partition.pivotHead) {
            partition.subListHead.next = sort(partition.pivotHead.next, tailNext);
            return partition.subListHead;
        } else {
            sort(partition.pivotHead, tailNext);
            return sort(partition.subListHead, partition.pivotHead);
        }
    }

    Partition partition(ListNode head, ListNode tailNext) {
        int pivot = head.val;
        ListNode subListDummyHead = new ListNode(0, head);
        ListNode subListHead = subListDummyHead;
        ListNode pivotDummyHead = new ListNode(0, head);
        ListNode pivotHead = pivotDummyHead;
        while (head != tailNext) {
            if (head.val < pivot) {
                subListHead.next = head;
                subListHead = subListHead.next;
            } else {
                pivotHead.next = head;
                pivotHead = pivotHead.next;
            }
            head = head.next;
        }
        subListHead.next = pivotDummyHead.next;
        return new Partition(subListDummyHead.next, pivotDummyHead.next, pivotHead);
    }
}
