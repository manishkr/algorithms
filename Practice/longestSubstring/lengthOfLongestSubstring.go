package main

import (
	"strings"
)

func max(x, y int) int {
	if x < y {
		return y
	}
	return x
}

func lengthOfLongestSubstring(s string) int {
	lsMap := make(map[string]int)

	strArray := strings.Split(s, "")
	longestLength := 0
	i := 0
	for j := 0; j < len(strArray); j++ {
		if val, ok := lsMap[strArray[j]]; ok {
			i = max(i, val)
		}
		longestLength = max(longestLength, j-i+1)
		lsMap[strArray[j]] = j + 1
	}

	return longestLength
}
