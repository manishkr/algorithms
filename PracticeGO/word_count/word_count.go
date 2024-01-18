package main

import (
	"golang.org/x/tour/wc"
	"strings"
)

func WordCount(s string) map[string]int {
	data := strings.Fields(s)
	word_count := make(map[string]int)
	for _, v := range data {
		count, ok := word_count[v]
		if ok {
			word_count[v] = count + 1
		} else {
			word_count[v] = 1
		}
	}
	return word_count
}

func main() {
	wc.Test(WordCount)
}
