package main

func numNeighbour(grid [][]int, i int, j int) int {
	count := 0
	rows := len(grid)
	columns := len(grid[0])

	if i > 0 && grid[i-1][j] == 1 {
		count += 1
	}

	if j > 0 && grid[i][j-1] == 1 {
		count += 1
	}

	if i < rows-1 && grid[i+1][j] == 1 {
		count += 1
	}

	if j < columns-1 && grid[i][j+1] == 1 {
		count += 1
	}

	return count
}
func islandPerimeter(grid [][]int) int {
	if grid == nil || len(grid) == 0 || len(grid[0]) == 0 {
		return 0
	}
	rows := len(grid)
	columns := len(grid[0])

	perimeter := 0
	for i := 0; i < rows; i++ {
		for j := 0; j < columns; j++ {
			if grid[i][j] == 1 {
				perimeter += 4 - numNeighbour(grid, i, j)
			}
		}
	}

	return perimeter
}
