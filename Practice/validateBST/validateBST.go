package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func flatTree(root *TreeNode) []int {
	if root == nil {
		return nil
	}

	result := append(flatTree(root.Left), []int{root.Val}...)
	result = append(result, flatTree(root.Right)...)
	return result
}

func checkOrder(input []int) bool {
	if input == nil || len(input) == 0 {
		return true
	}
	maxVal := input[0]
	for i := 1; i < len(input); i++ {
		if input[i] > maxVal {
			maxVal = input[i]
		} else {
			return false
		}
	}

	return true
}

func isValidBST(root *TreeNode) bool {
	result := flatTree(root)

	return checkOrder(result)
}
