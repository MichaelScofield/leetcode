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
    public List<List<Integer>> pathSum(TreeNode root, int sum) {
        if (root == null) {
            return new ArrayList<>(1);
        }
        List<List<Integer>> paths = new ArrayList<>();
        List<Integer> path = new ArrayList<>();
        path.add(root.val);
        pathSum(root, sum - root.val, path, paths);
        return paths;
    }

    void pathSum(TreeNode root, int sum, List<Integer> path, List<List<Integer>> paths) {
        if (sum == 0 && root.left == null && root.right == null) {
            paths.add(new ArrayList<>(path));
            return;
        }
        if (root.left != null) {
            path.add(root.left.val);
            pathSum(root.left, sum - root.left.val, path, paths);
            path.remove(path.size() - 1);
        }
        if (root.right != null) {
            path.add(root.right.val);
            pathSum(root.right, sum - root.right.val, path, paths);
            path.remove(path.size() - 1);
        }
    }
}
