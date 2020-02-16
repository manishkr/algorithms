package main

import "testing"

func TestNumberOfIslands(t *testing.T) {
	input1 := [][]byte{{1, 1, 1, 1, 0}, {1, 1, 0, 1, 0}, {1, 1, 0, 0, 0}, {0, 0, 0, 0, 0}}
	input2 := [][]byte{{1, 1, 0, 0, 0}, {1, 1, 0, 0, 0}, {0, 0, 1, 0, 0}, {0, 0, 0, 1, 1}}

	output1 := numIslands(input1)
	output2 := numIslands(input2)

	if output1 != 1 {
		t.Errorf("numIslands(input1) = %d; want 1", output1)
	}

	if output2 != 3 {
		t.Errorf("numIslands(input2) = %d; want 3", output2)
	}
}
