package main

func max(x int, y int) int {
	if x > y {
		return x
	}

	return y
}
func isValid(grid [][]int, visited [][]bool, i int, j int) bool {
	if i < 0 || i >= len(grid) || j < 0 || j >= len(grid[0]) || visited[i][j] || grid[i][j] == 0 {
		return false
	}

	return true
}

func dfs(grid [][]int, visited [][]bool, directions [][]int, i int, j int) int {
	area := 0
	for index := 0; index < len(directions); index++ {
		ni := i + directions[index][0]
		nj := j + directions[index][1]

		if isValid(grid, visited, ni, nj) {
			visited[ni][nj] = true
			area += 1
			area += dfs(grid, visited, directions, ni, nj)
		}
	}

	return area
}

func maxAreaOfIsland(grid [][]int) int {
	if nil == grid || len(grid) == 0 {
		return 0
	}
	directions := [][]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	rows := len(grid)
	columns := len(grid[0])
	visited := make([][]bool, rows)
	for i := 0; i < rows; i++ {
		visited[i] = make([]bool, columns)
	}

	maxAreaOfIsland := 0
	for i := 0; i < rows; i++ {
		for j := 0; j < columns; j++ {
			if grid[i][j] == 1 && !visited[i][j] {
				maxAreaOfIsland = max(maxAreaOfIsland, dfs(grid, visited, directions, i, j))
			}
		}
	}

	return maxAreaOfIsland
}
