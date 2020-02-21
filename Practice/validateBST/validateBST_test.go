package main

import "testing"

func TestSubArraySum(t *testing.T) {
	input1 := TreeNode{
		Val: 2,
		Left: &TreeNode{
			Val:   1,
			Left:  nil,
			Right: nil,
		},
		Right: &TreeNode{
			Val:   3,
			Left:  nil,
			Right: nil,
		},
	}

	output1 := isValidBST(&input1)

	if output1 != true {
		t.Errorf("isValidBST(input1) = %t; want true", output1)
	}

	input2 := TreeNode{
		Val: 5,
		Left: &TreeNode{
			Val:   1,
			Left:  nil,
			Right: nil,
		},
		Right: &TreeNode{
			Val: 4,
			Left: &TreeNode{
				Val:   3,
				Left:  nil,
				Right: nil,
			},
			Right: &TreeNode{
				Val:   6,
				Left:  nil,
				Right: nil,
			},
		},
	}

	output2 := isValidBST(&input2)

	if output2 != false {
		t.Errorf("isValidBST(input2) = %t; want false", output2)
	}

	output1 = isValidBSTNoSpace(&input1)
	if output1 != true {
		t.Errorf("isValidBSTNoSpace(input1) = %t; want true", output1)
	}

	output2 = isValidBSTNoSpace(&input2)

	if output2 != false {
		t.Errorf("isValidBSTNoSpace(input2) = %t; want false", output2)
	}
}
