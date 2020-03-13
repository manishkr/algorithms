package longestPlaindromeSubseq

func max(x int, y int) int {
	if x > y {
		return x
	}

	return y
}
func longestPalindromeSubseqHelper(s string, i int, j int, memMap [][]int) int {
	if i == j {
		return 1
	}

	if memMap[i][j] > -1 {
		return memMap[i][j]
	}

	if s[i] == s[j] {
		val := 2
		if i+1 != j {
			val = 2 + longestPalindromeSubseqHelper(s, i+1, j-1, memMap)
		}
		memMap[i][j] = val
	} else {
		memMap[i][j] = max(longestPalindromeSubseqHelper(s, i+1, j, memMap), longestPalindromeSubseqHelper(s, i, j-1, memMap))
	}

	return memMap[i][j]
}

func longestPalindromeSubseq(s string) int {
	memMap := make([][]int, len(s))
	for i := 0; i < len(s); i++ {
		memMap[i] = make([]int, len(s))
	}

	for i := 0; i < len(s); i++ {
		for j := 0; j < len(s); j++ {
			memMap[i][j] = -1
		}
	}

	return longestPalindromeSubseqHelper(s, 0, len(s)-1, memMap)
}
