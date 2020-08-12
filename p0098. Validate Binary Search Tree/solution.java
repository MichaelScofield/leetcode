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
    public boolean isValidBST(TreeNode root) {
        if (root == null) {
            return true;
        }
        if (isNotValidBst(root)) {
            return false;
        }
        if (root.left != null) {
            TreeNode larger = findLarger(root.left);
            if (larger == null) {
                return false;
            }
            if (root.val <= larger.val) {
                return false;
            }
        }
        if (root.right != null) {
            TreeNode smaller = findSmaller(root.right);
            if (smaller == null) {
                return false;
            }
            if (root.val >= smaller.val) {
                return false;
            }
        }
        return isValidBST(root.left) && isValidBST(root.right);
    }

    TreeNode findLarger(TreeNode node) {
        if (isNotValidBst(node)) {
            return null;
        }
        if (node.right != null) {
            return findLarger(node.right);
        }
        return node;
    }

    TreeNode findSmaller(TreeNode node) {
        if (isNotValidBst(node)) {
            return null;
        }
        if (node.left != null) {
            return findSmaller(node.left);
        }
        return node;
    }

    boolean isNotValidBst(TreeNode node) {
        return node.right != null && node.right.val <= node.val ||
                node.left != null && node.left.val >= node.val;
    }
}
