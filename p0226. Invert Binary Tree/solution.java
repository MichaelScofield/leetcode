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
    
    private Queue<TreeNode> queue = new LinkedList<TreeNode>();
    
    public TreeNode invertTree(TreeNode root) {
        if (root == null) {
            return null;
        }
        TreeNode node = root;
        do {
            TreeNode left = node.left;
            if (left != null) {
                queue.offer(left);
            }
            TreeNode right = node.right;
            if (right != null) {
                queue.offer(right);
            }
            node.right = left;
            node.left = right;
        } while ((node = queue.poll()) != null);
        return root;
    }
}
