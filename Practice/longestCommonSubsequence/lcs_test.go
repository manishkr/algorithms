package main

import "testing"

func TestLCS(t *testing.T) {
	input11 := "abcde"
	input12 := "ace"

	input21 := "abc"
	input22 := "abc"

	input31 := "abc"
	input32 := "def"

	input41 := "hieroglyphology"
	input42 := "michaelangelo"

	output := longestCommonSubsequence(input11, input12)
	if output != 3 {
		t.Errorf("longestCommonSubsequence(input11, input12) = %d; want 3", output)
	}

	output = longestCommonSubsequence(input21, input22)
	if output != 3 {
		t.Errorf("longestCommonSubsequence(input21, input22) = %d; want 3", output)
	}

	output = longestCommonSubsequence(input31, input32)
	if output != 0 {
		t.Errorf("longestCommonSubsequence(input31, input32) = %d; want 0", output)
	}

	output = longestCommonSubsequence(input41, input42)
	if output != 5 {
		t.Errorf("longestCommonSubsequence(input41, input42) = %d; want 5", output)
	}

	output = longestCommonSubsequenceBottomUp(input11, input12)
	if output != 3 {
		t.Errorf("longestCommonSubsequenceBottomUp(input11, input12) = %d; want 3", output)
	}

	output = longestCommonSubsequenceBottomUp(input21, input22)
	if output != 3 {
		t.Errorf("longestCommonSubsequenceBottomUp(input21, input22) = %d; want 3", output)
	}

	output = longestCommonSubsequenceBottomUp(input31, input32)
	if output != 0 {
		t.Errorf("longestCommonSubsequenceBottomUp(input31, input32) = %d; want 0", output)
	}

	output = longestCommonSubsequenceBottomUp(input41, input42)
	if output != 5 {
		t.Errorf("longestCommonSubsequenceBottomUp(input41, input42) = %d; want 5", output)
	}

}
