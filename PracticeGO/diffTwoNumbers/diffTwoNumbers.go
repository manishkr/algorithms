package main

import (
	"fmt"
	"math"
)

func diff(s1 []int, s2 []int, num int) int {
	var res []int
	for _, r1 := range s1 {
		found := false
		for _, r2 := range s2 {
			if int(math.Abs(float64(r1-r2))) <= num {
				found = true
				continue
			}
		}

		if !found {
			res = append(res, r1)
		}

	}

	return len(res)
}
func main() {
	s1 := []int{2, 3, 4}
	s2 := []int{12, 12, 9, 13}

	res := diff(s1, s2, 2)
	fmt.Println(res)
}
