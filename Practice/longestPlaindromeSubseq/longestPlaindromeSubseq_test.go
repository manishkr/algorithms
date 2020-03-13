package longestPlaindromeSubseq

import "testing"

func TestLongestPlaindromeSubSeq(t *testing.T) {
	input := "bbbab"
	output := longestPalindromeSubseq(input)
	if output != 4 {
		t.Errorf("longestPalindromeSubseq(%s) = %d; want 4", input, output)
	}

	input = "bbbbb"
	output = longestPalindromeSubseq(input)
	if output != 5 {
		t.Errorf("longestPalindromeSubseq(%s) = %d; want 5", input, output)
	}

	input = "cbbd"
	output = longestPalindromeSubseq(input)
	if output != 2 {
		t.Errorf("longestPalindromeSubseq(%s) = %d; want 2", input, output)
	}
}
