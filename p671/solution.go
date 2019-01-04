/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func findSecondMinimumValue(root *TreeNode) int {
	smallest, secondSmallest := -1, -1

	c := make(chan int)
	go iterateTree(root, c)

	for i := range c {
		if smallest < 0 || i <= smallest {
			smallest = i
		} else if secondSmallest < 0 || i < secondSmallest {
			secondSmallest = i
		}
	}

	return secondSmallest
}

func iterateTree(root *TreeNode, c chan int) {
	queue := make(chan *TreeNode, 1024)
	queue <- root
	for {
		select {
		case node := <-queue:
			c <- node.Val

			if node.Left != nil {
				queue <- node.Left
			}
			if node.Right != nil {
				queue <- node.Right
			}
		default:
			close(c)
			return
		}
	}
}
