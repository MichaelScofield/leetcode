import (
    "strconv"
)

func tree2str(t *TreeNode) string {
    str := ""
    return preorder(t, &str)
}

func preorder(t *TreeNode, str *string) string {
    if t == nil {
        return ""
    }
    *str += strconv.Itoa(t.Val)
    if t.Left != nil {
        *str += "("
        preorder(t.Left, str)
        *str += ")"
    } else {
        if t.Right != nil {
            *str += "()"
        }
    }
    if t.Right != nil {
        *str += "("
        preorder(t.Right, str)
        *str += ")"
    }
    return *str
}
