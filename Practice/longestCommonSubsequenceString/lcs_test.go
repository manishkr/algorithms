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
	if output != "ace" {
		t.Errorf("longestCommonSubsequence(input11, input12) = %s; want ace", output)
	}

	output = longestCommonSubsequence(input21, input22)
	if output != "abc" {
		t.Errorf("longestCommonSubsequence(input21, input22) = %s; want abc", output)
	}

	output = longestCommonSubsequence(input31, input32)
	if output != "" {
		t.Errorf("longestCommonSubsequence(input31, input32) = %s; want nil", output)
	}

	output = longestCommonSubsequence(input41, input42)
	if output != "hello" && output != "heglo" && output != "iello" {
		t.Errorf("longestCommonSubsequence(input41, input42) = %s; want iello or hello or heglo", output)
	}

	output = longestCommonSubsequenceBottomUp(input11, input12)
	if output != "ace" {
		t.Errorf("longestCommonSubsequenceBottomUp(input11, input12) = %s; want ace", output)
	}

	output = longestCommonSubsequenceBottomUp(input21, input22)
	if output != "abc" {
		t.Errorf("longestCommonSubsequenceBottomUp(input21, input22) = %s; want abc", output)
	}

	output = longestCommonSubsequenceBottomUp(input31, input32)
	if output != "" {
		t.Errorf("longestCommonSubsequenceBottomUp(input31, input32) = %s; want nil", output)
	}

	output = longestCommonSubsequenceBottomUp(input41, input42)
	if output != "hello" && output != "heglo" && output != "iello" {
		t.Errorf("longestCommonSubsequence(input41, input42) = %s; want iello or hello or heglo", output)
	}
}
