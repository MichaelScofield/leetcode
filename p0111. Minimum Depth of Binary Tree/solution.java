/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode(int x) { val = x; }
 * }
 */
class Solution {
    public int minDepth(TreeNode root) {
        return root == null ? 0 : minDepth(root, 1);
    }

    private int minDepth(TreeNode node, int currentDepth) {
        if (node.left == null && node.right == null) {
            return currentDepth;
        }
        int minDepthL = node.left != null ?
                minDepth(node.left, currentDepth + 1) : Integer.MAX_VALUE;
        int minDepthR = node.right != null ?
                minDepth(node.right, currentDepth + 1) : Integer.MAX_VALUE;
        return Math.min(minDepthL, minDepthR);
    }
}
