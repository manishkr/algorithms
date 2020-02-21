package main

func longestCommonSubsequenceBottomUp(text1 string, text2 string) int {
	lcsMap := make([][]int, len(text1)+1)
	for i := 0; i < len(text1)+1; i++ {
		lcsMap[i] = make([]int, len(text2)+1)
	}

	for i := 0; i <= len(text1); i++ {
		for j := 0; j <= len(text2); j++ {
			if i != 0 && j != 0 {
				if text1[i-1] == text2[j-1] {
					lcsMap[i][j] = 1 + lcsMap[i-1][j-1]
				} else {
					lcsMap[i][j] = max(lcsMap[i][j-1], lcsMap[i-1][j])
				}
			}
		}
	}
	return lcsMap[len(text1)][len(text2)]
}
