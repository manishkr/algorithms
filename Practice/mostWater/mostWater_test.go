package main

import (
	"testing"
)

func TestMostWater(t *testing.T) {
	input := []int{1, 8, 6, 2, 5, 4, 8, 3, 7}
	output := maxArea(input)
	if output != 49 {
		t.Errorf("maxArea(input) = %d; want 49", output)
	}

	input = []int{1, 8}
	output = maxArea(input)
	if output != 1 {
		t.Errorf("maxArea(input) = %d; want 1", output)
	}
}
