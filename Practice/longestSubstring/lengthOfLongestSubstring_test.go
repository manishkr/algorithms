package main

import "testing"

func TestLenghtOfLongestSubstring(t *testing.T) {
	input := "abcabcbb"
	output := lengthOfLongestSubstring(input)
	if output != 3 {
		t.Errorf("lengthOfLongestSubstring(input) = %d; want 3", output)
	}

	input = "bbbbb"
	output = lengthOfLongestSubstring(input)
	if output != 1 {
		t.Errorf("lengthOfLongestSubstring(input) = %d; want 1", output)
	}

	input = "pwwkew"
	output = lengthOfLongestSubstring(input)
	if output != 3 {
		t.Errorf("lengthOfLongestSubstring(input) = %d; want 3", output)
	}
}
