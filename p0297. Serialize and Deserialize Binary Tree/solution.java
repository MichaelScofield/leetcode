/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode(int x) { val = x; }
 * }
 */
public class Codec {

    public String serialize(TreeNode root) {
        if (root == null) {
            return "";
        }
        StringBuilder sb = new StringBuilder();
        serialize(root, sb);
        return sb.toString();
    }

    void serialize(TreeNode node, StringBuilder sb) {
        if (node == null) {
            sb.append("x ");
            return;
        }
        sb.append(node.val).append(" ");
        serialize(node.left, sb);
        serialize(node.right, sb);
    }

    public TreeNode deserialize(String data) {
        if (data == null || data.isEmpty()) {
            return null;
        }
        LinkedList<String> vals = Arrays.stream(data.trim().split(" ")).collect(LinkedList::new, LinkedList::offer, LinkedList::addAll);
        return deserialize(vals);
    }

    TreeNode deserialize(LinkedList<String> vals) {
        if (vals.size() == 0) {
            return null;
        }
        String val = vals.poll();
        if (val.equals("x")) {
            return null;
        }
        TreeNode node = new TreeNode(Integer.parseInt(val));
        node.left = deserialize(vals);
        node.right = deserialize(vals);
        return node;
    }
}

// Your Codec object will be instantiated and called as such:
// Codec codec = new Codec();
// codec.deserialize(codec.serialize(root));
