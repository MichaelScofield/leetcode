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
    public int sumOfLeftLeaves(TreeNode root) {
        if (root == null) {
            return 0;
        }
        Queue<TreeNode> q = new LinkedList<TreeNode>();
        int sum = 0;
        do {
            TreeNode node = root.left;
            if (node != null) {
                q.offer(node);

                if (node.left == null && node.right == null) {
                    sum += node.val;
                }
            }
            node = root.right;
            if (node != null) {
                q.offer(node);
            }
        } while ((root = q.poll()) != null);
        return sum;
    }
}
