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
    public List<List<Integer>> levelOrder(TreeNode root) {
        List<List<Integer>> levels = new ArrayList<>();
        if (root == null) {
            return levels;
        }
        Queue<TreeNode> queue = new LinkedList<>();
        queue.add(root);
        int currChildren = 1;
        while (!queue.isEmpty()) {
            int nextChildren = 0;
            List<Integer> level = new ArrayList<>();
            for (int i = 0; i < currChildren; i++) {
                TreeNode node = queue.poll();
                assert node != null;
                level.add(node.val);
                if (node.left != null) {
                    nextChildren += 1;
                    queue.add(node.left);
                }
                if (node.right != null) {
                    nextChildren += 1;
                    queue.add(node.right);
                }
            }
            currChildren = nextChildren;
            levels.add(level);
        }
        return levels;
    }
}
