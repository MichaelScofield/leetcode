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
    public boolean isSubStructure(TreeNode A, TreeNode B) {
        if (A == null || B == null) {
            return false;
        }
        if (isSameLargerTree(A, B)) {
            return true;
        }
        return isSubStructure(A.left, B) || isSubStructure(A.right, B);
    }

    boolean isSameLargerTree(TreeNode ra, TreeNode rb) {
        if (rb == null) {
            return true;
        }
        if (ra == null) {
            return false;
        }
        if (ra.val != rb.val) {
            return false;
        }
        return isSameLargerTree(ra.left, rb.left) && isSameLargerTree(ra.right, rb.right);
    }
}
