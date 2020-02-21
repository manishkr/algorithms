package main

func max(x, y int) int {
	if x < y {
		return y
	}
	return x
}

func lcsHelper(text1 string, text2 string, index1 int, index2 int, lcsMap [][]int) int {
	if index1 >= len(text1) || index2 >= len(text2) {
		return 0
	}

	value := lcsMap[index1][index2]
	if value >= 0 {
		return value
	}

	if text1[index1] == text2[index2] {
		value = 1 + lcsHelper(text1, text2, index1+1, index2+1, lcsMap)
	} else {
		value = max(lcsHelper(text1, text2, index1, index2+1, lcsMap), lcsHelper(text1, text2, index1+1, index2, lcsMap))
	}

	lcsMap[index1][index2] = value

	return value
}

func longestCommonSubsequence(text1 string, text2 string) int {
	lcsMap := make([][]int, len(text1))
	for i := 0; i < len(text1); i++ {
		lcsMap[i] = make([]int, len(text2))
	}

	for i := range lcsMap {
		for j := range lcsMap[i] {
			lcsMap[i][j] = -1
		}
	}
	return lcsHelper(text1, text2, 0, 0, lcsMap)
}
