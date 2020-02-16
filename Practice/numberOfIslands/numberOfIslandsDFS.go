package main

func dfs(grid [][]byte, visited [][]bool, directions [][]int, i int, j int) {
	for index := 0; index < len(directions); index++ {
		ni := i + directions[index][0]
		nj := j + directions[index][1]

		if isValid(grid, visited, ni, nj) {
			visited[ni][nj] = true
			dfs(grid, visited, directions, ni, nj)
		}
	}
}

func numIslandsDFS(grid [][]byte) int {
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

	numOfIslands := 0
	for i := 0; i < rows; i++ {
		for j := 0; j < columns; j++ {
			if grid[i][j] == '1' && !visited[i][j] {
				numOfIslands += 1
				dfs(grid, visited, directions, i, j)
			}
		}
	}

	return numOfIslands
}
