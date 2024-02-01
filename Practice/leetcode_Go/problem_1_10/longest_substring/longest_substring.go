package main

import "fmt"

func lengthOfLongestSubstring(s string) int {
	var maxLen = 0
	var start = 0
	var charMap = make(map[byte]int)

	for i := 0; i < len(s); i++ {
		if charMap[s[i]] > 0 && charMap[s[i]] > start {
			start = charMap[s[i]]
		}
		if i-start+1 > maxLen {
			maxLen = i - start + 1
		}
		charMap[s[i]] = i + 1
	}
	return maxLen
}

func main() {
	s := "abcabcbb"
	fmt.Println(lengthOfLongestSubstring(s))

	s = "bbbbb"
	fmt.Println(lengthOfLongestSubstring(s))

	s = "pwwkew"
	fmt.Println(lengthOfLongestSubstring(s))
}
