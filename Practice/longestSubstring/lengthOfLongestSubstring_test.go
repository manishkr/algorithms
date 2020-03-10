package main

import "testing"

func TestLenghtOfLongestSubstring(t *testing.T) {
	input := "abcabcbb"
	output := lengthOfLongestSubstring(input)
	if output != 3 {
		t.Errorf("lengthOfLongestSubstring(%s) = %d; want 3", input, output)
	}

	input = "bbbbb"
	output = lengthOfLongestSubstring(input)
	if output != 1 {
		t.Errorf("lengthOfLongestSubstring(%s) = %d; want 1", input, output)
	}

	input = "pwwkew"
	output = lengthOfLongestSubstring(input)
	if output != 3 {
		t.Errorf("lengthOfLongestSubstring(%s) = %d; want 3", input, output)
	}
}
