package main

func lcsHelper(text1 string, text2 string, index1 int, index2 int, lcsMap [][]string) string {
	if index1 >= len(text1) || index2 >= len(text2) {
		return ""
	}

	value := lcsMap[index1][index2]
	if value != "" {
		return value
	}

	if text1[index1] == text2[index2] {
		value = string(text1[index1]) + lcsHelper(text1, text2, index1+1, index2+1, lcsMap)
	} else {
		l1 := lcsHelper(text1, text2, index1, index2+1, lcsMap)
		l2 := lcsHelper(text1, text2, index1+1, index2, lcsMap)

		if len(l1) > len(l2) {
			value = l1
		} else {
			value = l2
		}
	}

	lcsMap[index1][index2] = value

	return value
}

func longestCommonSubsequence(text1 string, text2 string) string {
	lcsMap := make([][]string, len(text1))
	for i := 0; i < len(text1); i++ {
		lcsMap[i] = make([]string, len(text2))
	}

	for i := range lcsMap {
		for j := range lcsMap[i] {
			lcsMap[i][j] = ""
		}
	}
	return lcsHelper(text1, text2, 0, 0, lcsMap)
}
