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

func createSubString(s []string, index int) int {
	set := make(map[string]bool)

	for i := index; i < len(s); i++ {
		if _, ok := set[s[i]]; ok {
			return len(set)
		} else {
			set[s[i]] = true
		}
	}

	return len(set)
}

func lengthOfLongestSubstring(s string) int {
	lsMap := make([]map[string]bool, len(s))
	for i := 0; i < len(s); i++ {
		lsMap[i] = make(map[string]bool)
	}

	strArray := strings.Split(s, "")
	longestLength := 0
	for i := 0; i < len(s); i++ {
		longestLength = max(longestLength, createSubString(strArray, i))
	}

	return longestLength
}
