package main

import "testing"

func TestPerimeterIsland(t *testing.T) {
	input1 := [][]int{{0, 1, 0, 0},
		{1, 1, 1, 0},
		{0, 1, 0, 0},
		{1, 1, 0, 0}}

	input2 := [][]int{{0, 0, 0, 0, 0, 0, 0, 0}}
	input3 := [][]int{{1}}

	output1 := islandPerimeter(input1)
	output2 := islandPerimeter(input2)
	output3 := islandPerimeter(input3)

	if output1 != 16 {
		t.Errorf("maxAreaOfIsland(input1) = %d; want 16", output1)
	}

	if output2 != 0 {
		t.Errorf("maxAreaOfIsland(input2) = %d; want 0", output2)
	}

	if output3 != 4 {
		t.Errorf("maxAreaOfIsland(input3) = %d; want 4", output3)
	}
}
