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
    
    int k;

    public int kthSmallest(TreeNode root, int k) {
        assert k >= 1;
        this.k = k;
        return kthSmallest(root).val;
    }

    TreeNode kthSmallest(TreeNode root) {
        if (root == null) {
            return null;
        }
        if (root.left == null && root.right == null) {
            return --k == 0 ? root : null;
        }
        if (root.left != null) {
            TreeNode node = kthSmallest(root.left);
            if (node != null) {
                return node;
            }
        }
        if (--k == 0) {
            return root;
        }
        return kthSmallest(root.right);
    }
}
