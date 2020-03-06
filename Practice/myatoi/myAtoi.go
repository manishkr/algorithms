package main

import (
	"math"
	"strings"
)

func myAtoi(str string) int {
	strIntMap := map[string]int{
		"0": 0,
		"1": 1,
		"2": 2,
		"3": 3,
		"4": 4,
		"5": 5,
		"6": 6,
		"7": 7,
		"8": 8,
		"9": 9,
	}
	str = strings.TrimLeft(str, " ")
	strArr := strings.Split(str, "")
	if len(strArr) == 0 {
		return 0
	}

	multiplier := 1
	number := 0
	index := 0
	if strArr[index] == "+" {
		index = 1
	} else if strArr[index] == "-" {
		multiplier = -1
		index = 1
	}

	for i := index; i < len(strArr); i++ {
		value, ok := strIntMap[strArr[i]]
		if ok {
			if number > math.MaxInt32/10 || (number == math.MaxInt32/10 && value > 7) {
				if multiplier == 1 {
					return math.MaxInt32
				} else {
					return math.MinInt32
				}
			}
			if strArr[i] == "+" {
				multiplier = 1
			} else if strArr[i] == "-" {
				multiplier = -1
			} else {
				number = number*10 + value
			}
		} else {
			break
		}
	}
	return multiplier * number
}
