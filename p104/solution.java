/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode(int x) { val = x; }
 * }
 */
public class Solution {
    public int maxDepth(TreeNode root) {
        if (root == null) {
            return 0;
        }
        int maxDepthL = maxDepth(root.left);
        int maxDepthR = maxDepth(root.right);
        return 1 + (maxDepthL > maxDepthR ? maxDepthL : maxDepthR);
    }
}
