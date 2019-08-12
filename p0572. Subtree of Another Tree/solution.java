/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode(int x) { val = x; }
 * }
 */
import java.util.LinkedList;
import java.util.Queue;
class Solution {
    public boolean isSubtree(TreeNode s, TreeNode t) {
        if (s == null && t == null) {
            return true;
        }
        if (s != null && t != null) {
            return s.val == t.val && isSameTree(s, t) || isSubtree(s.left, t) || isSubtree(s.right, t);
        }
        return false;
    }
    
    // https://github.com/MichaelScofield/leetcode/blob/master/p0100.%20Same%20Tree/solution.java
    private boolean isSameTree(TreeNode p, TreeNode q) {
        if (p == q) {
            return true;
        }
        if (!(p != null & q != null)) {
            return false;
        }
        Queue<TreeNode> l1 = new LinkedList<>();
        Queue<TreeNode> l2 = new LinkedList<>();
        do {
            if (p.val != q.val) {
                return false;
            }

            if (p.left != null) {
                if (q.left != null) {
                    l1.offer(p.left);
                    l2.offer(q.left);
                } else {
                    return false;
                }
            } else {
                if (q.left != null) {
                    return false;
                }
            }

            if (p.right != null) {
                if (q.right != null) {
                    l1.offer(p.right);
                    l2.offer(q.right);
                } else {
                    return false;
                }
            } else {
                if (q.right != null) {
                    return false;
                }
            }
        } while ((p = l1.poll()) != null && (q = l2.poll()) != null);
        return true;
    }
}
