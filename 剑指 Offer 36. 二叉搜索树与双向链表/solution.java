/*
// Definition for a Node.
class Node {
    public int val;
    public Node left;
    public Node right;

    public Node() {}

    public Node(int _val) {
        val = _val;
    }

    public Node(int _val,Node _left,Node _right) {
        val = _val;
        left = _left;
        right = _right;
    }
};
*/
class Solution {
    
    Node lastNode;

    public Node treeToDoublyList(Node root) {
        if (root == null) {
            return null;
        }
        Node head = root;
        Node tail = root;
        convert(root);
        while (head.left != null) {
            head = head.left;
        }
        while (tail.right != null) {
            tail = tail.right;
        }
        head.left = tail;
        tail.right = head;
        return head;
    }

    void convert(Node node) {
        if (node.left != null) {
            convert(node.left);
        }
        if (lastNode != null) {
            node.left = lastNode;
            lastNode.right = node;
        }
        lastNode = node;
        if (node.right != null) {
            convert(node.right);
        }
    }
}
