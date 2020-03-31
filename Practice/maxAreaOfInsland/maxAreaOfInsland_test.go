package main

import "testing"

func TestNumberOfIslands(t *testing.T) {
	input1 := [][]int{{0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0},
		{0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0},
		{0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0},
		{0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0},
		{0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0},
		{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0},
		{0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0},
		{0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0}}

	input2 := [][]int{{0, 0, 0, 0, 0, 0, 0, 0}}

	output1 := maxAreaOfIsland(input1)
	output2 := maxAreaOfIsland(input2)

	if output1 != 6 {
		t.Errorf("maxAreaOfIsland(input1) = %d; want 6", output1)
	}

	if output2 != 0 {
		t.Errorf("maxAreaOfIsland(input2) = %d; want 0", output2)
	}
}
