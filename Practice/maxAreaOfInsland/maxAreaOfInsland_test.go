package main

import "testing"

func TestMaxAreaOfIslands(t *testing.T) {
	input1 := [][]int{{0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0},
		{0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0},
		{0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0},
		{0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0},
		{0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0},
		{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0},
		{0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0},
		{0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0}}

	input2 := [][]int{{0, 0, 0, 0, 0, 0, 0, 0}}
	input3 := [][]int{{1}}

	output1 := maxAreaOfIsland(input1)
	output2 := maxAreaOfIsland(input2)
	output3 := maxAreaOfIsland(input3)

	if output1 != 6 {
		t.Errorf("maxAreaOfIsland(input1) = %d; want 6", output1)
	}

	if output2 != 0 {
		t.Errorf("maxAreaOfIsland(input2) = %d; want 0", output2)
	}

	if output3 != 1 {
		t.Errorf("maxAreaOfIsland(input3) = %d; want 1", output3)
	}
}
