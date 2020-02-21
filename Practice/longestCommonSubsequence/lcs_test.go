package main

import "testing"

func TestLCS(t *testing.T) {
	input1 := "abcde"
	input2 := "ace"

	output := longestCommonSubsequence(input1, input2)
	if output != 3 {
		t.Errorf("longestCommonSubsequence(input1, input2) = %d; want 3", output)
	}

	input1 = "abc"
	input2 = "abc"

	output = longestCommonSubsequence(input1, input2)
	if output != 3 {
		t.Errorf("longestCommonSubsequence(input1, input2) = %d; want 3", output)
	}

	input1 = "abc"
	input2 = "def"

	output = longestCommonSubsequence(input1, input2)
	if output != 0 {
		t.Errorf("longestCommonSubsequence(input1, input2) = %d; want 0", output)
	}

	input1 = "hieroglyphology"
	input2 = "michaelangelo"

	output = longestCommonSubsequence(input1, input2)
	if output != 5 {
		t.Errorf("longestCommonSubsequence(input1, input2) = %d; want 5", output)
	}
}
