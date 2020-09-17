/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */
class Solution {
    public boolean isBalanced(TreeNode root) {
        return isBalanced(root, new AtomicInteger(0));
    }

    boolean isBalanced(TreeNode root, AtomicInteger depth) {
        if (root == null) {
            depth.set(0);
            return true;
        }
        AtomicInteger leftDepth = new AtomicInteger(0);
        AtomicInteger rightDepth = new AtomicInteger(0);
        if (isBalanced(root.left, leftDepth) && isBalanced(root.right, rightDepth)) {
            if (Math.abs(leftDepth.get() - rightDepth.get()) <= 1) {
                depth.set(1 + Math.max(leftDepth.get(), rightDepth.get()));
                return true;
            }
        }
        return false;
    }
}
