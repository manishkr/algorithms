package main

import "testing"

func TestMinimumInArray(t *testing.T) {
	input := []int{3, 4, 5, 1, 2}
	output := findMin(input)

	if output != 1 {
		t.Errorf("findMin(input) = %d; want 1", output)
	}

	input = []int{4, 5, 6, 7, 0, 1, 2}
	output = findMin(input)

	if output != 0 {
		t.Errorf("findMin(input) = %d; want 0", output)
	}
}
