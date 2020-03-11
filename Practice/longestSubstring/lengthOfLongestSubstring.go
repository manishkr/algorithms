package main

func max(x, y int) int {
	if x < y {
		return y
	}
	return x
}

func lengthOfLongestSubstring(s string) int {
	lsMap := make(map[rune]int)
	longestLength := 0
	i := 0
	for pos, char := range s {
		if val, ok := lsMap[char]; ok {
			i = max(i, val)
		}
		longestLength = max(longestLength, pos-i+1)
		lsMap[char] = pos + 1
	}

	return longestLength
}
