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
    public TreeNode buildTree(int[] preorder, int[] inorder) {
        if (preorder == null || inorder == null
                || preorder.length != inorder.length
                || preorder.length < 1) {
            return null;
        }
        return buildTree(preorder, 0, preorder.length, inorder, 0, inorder.length);
    }

    TreeNode buildTree(int[] preorder, int p, int q, int[] inorder, int x, int y) {
        if (p >= q || x >= y) {
            return null;
        }
        TreeNode root = new TreeNode(preorder[p]);
        int i;
        for (i = x; i < y; i++) {
            if (inorder[i] == preorder[p]) {
                break;
            }
        }
        root.left = buildTree(preorder, p + 1, p + 1 + (i - x), inorder, x, i);
        root.right = buildTree(preorder, p + 1 + (i - x), q, inorder, i + 1, y);
        return root;
    }
}
