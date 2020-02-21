package main

type Pair struct {
	i, j int
}

func max(x, y int) int {
	if x < y {
		return y
	}
	return x
}

func lcsHelper(text1 string, text2 string, index1 int, index2 int, lcsMap map[Pair]int) int {
	if index1 >= len(text1) {
		return 0
	}
	if index2 >= len(text2) {
		return 0
	}

	value, ok := lcsMap[Pair{
		i: index1,
		j: index2,
	}]

	if ok {
		return value
	}

	if text1[index1] == text2[index2] {
		value = 1 + lcsHelper(text1, text2, index1+1, index2+1, lcsMap)
	} else {
		value = max(lcsHelper(text1, text2, index1, index2+1, lcsMap), lcsHelper(text1, text2, index1+1, index2, lcsMap))
	}

	lcsMap[Pair{index1, index2}] = value

	return value
}

func longestCommonSubsequence(text1 string, text2 string) int {
	lcsMap := make(map[Pair]int)

	return lcsHelper(text1, text2, 0, 0, lcsMap)
}
