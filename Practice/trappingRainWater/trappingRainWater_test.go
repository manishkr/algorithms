package main

import (
	"testing"
)

func TestTrappingRainWater(t *testing.T) {
	input := []int{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1}
	output := trap(input)
	if output != 6 {
		t.Errorf("trap(input) = %d; want 6", output)
	}

	input = []int{1, 0, 1}
	output = trap(input)
	if output != 1 {
		t.Errorf("trap(input) = %d; want 1", output)
	}
}
