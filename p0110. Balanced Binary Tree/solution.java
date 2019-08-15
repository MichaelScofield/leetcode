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
    public boolean isBalanced(TreeNode root) {
        if (root == null) {
            return true;
        }
        if (Math.abs(maxDepth(root.left) - maxDepth(root.right)) > 1) {
            return false;
        }
        return isBalanced(root.left) && isBalanced(root.right);
    }

    // https://github.com/MichaelScofield/leetcode/blob/master/p0104.%20Maximum%20Depth%20of%20Binary%20Tree/solution.java
    private int maxDepth(TreeNode root) {
        if (root == null) {
            return 0;
        }
        int maxDepthL = maxDepth(root.left);
        int maxDepthR = maxDepth(root.right);
        return 1 + Math.max(maxDepthL, maxDepthR);
    }
}
