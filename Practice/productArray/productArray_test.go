package main

import (
	"testing"
)

func Equal(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i, v := range a {
		if v != b[i] {
			return false
		}
	}
	return true
}

func TestProductArray(t *testing.T) {
	input := []int{1, 2, 3, 4}
	output := productExceptSelf(input)
	if !Equal(output, []int{24, 12, 8, 6}) {
		t.Errorf("productExceptSelf(input) = %d; 24, 12, 8, 6} ", output)
	}
}
