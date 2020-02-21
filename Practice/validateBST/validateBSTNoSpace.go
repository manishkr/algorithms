package main

func checkTree(root *TreeNode, lower *int, upper *int) bool {
	if root == nil {
		return true
	}
	val := root.Val
	if lower != nil && val <= *lower {
		return false
	}

	if upper != nil && val >= *upper {
		return false
	}

	if !checkTree(root.Right, &val, upper) {
		return false
	}
	if !checkTree(root.Left, lower, &val) {
		return false
	}

	return true
}
func isValidBSTNoSpace(root *TreeNode) bool {
	return checkTree(root, nil, nil)
}
