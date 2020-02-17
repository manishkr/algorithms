package main

import "testing"

func TestSubArraySum(t *testing.T) {
	input1 := []int{1, 1, 1}
	input2 := []int{2, 4, 6, 10}
	input3 := []int{}

	output1 := subarraySum(input1, 2)
	output2 := subarraySum(input2, 16)
	output3 := subarraySum(input3, 0)

	if output1 != 2 {
		t.Errorf("subarraySum(input1) = %d; want 2", output1)
	}

	if output2 != 1 {
		t.Errorf("subarraySum(input2) = %d; want 1", output2)
	}

	if output3 != 0 {
		t.Errorf("subarraySum(input3) = %d; want 0", output3)
	}
}
