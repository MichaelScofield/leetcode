/*
// Definition for a Node.
class Node {
    int val;
    Node next;
    Node random;

    public Node(int val) {
        this.val = val;
        this.next = null;
        this.random = null;
    }
}
*/

class Solution {
    public Node copyRandomList(Node head) {
        if (head == null) {
            return null;
        }
        linkedClone(head);
        linkRandoms(head);
        return splitList(head);
    }

    void linkedClone(Node head) {
        while (head != null) {
            Node node = new Node(head.val);
            node.next = head.next;
            head.next = node;
            head = node.next;
        }
    }

    void linkRandoms(Node head) {
        while (head != null) {
            if (head.random != null) {
                head.next.random = head.random.next;
            }
            head = head.next.next;
        }
    }

    Node splitList(Node head) {
        Node dummy1 = new Node(0);
        Node dummy2 = new Node(0);
        Node newHead = dummy2;
        int i = 0;
        while (head != null) {
            if (i++ % 2 == 0) {
                dummy1.next = head;
                dummy1 = head;
            } else {
                dummy2.next = head;
                dummy2 = head;
            }
            head = head.next;
        }
        dummy1.next = null;
        return newHead.next;
    }
}
