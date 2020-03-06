package main

import (
	"fmt"
	"testing"
)

func TestPermutations(t *testing.T) {
	input1 := []int{1, 2, 3}
	input2 := []int{5}

	output := permute(input1)
	fmt.Println(output)

	output = permute(input2)
	fmt.Println(output)
}
