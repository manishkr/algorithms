package main

func isValidDFS(grid [][]byte, i int, j int) bool {
	if i < 0 || i >= len(grid) || j < 0 || j >= len(grid[0]) || grid[i][j] == '0' {
		return false
	}

	return true
}

func dfsInMemory(grid [][]byte, directions [][]int, i int, j int) {
	for index := 0; index < len(directions); index++ {
		ni := i + directions[index][0]
		nj := j + directions[index][1]

		if isValidDFS(grid, ni, nj) {
			grid[ni][nj] = '0'
			dfsInMemory(grid, directions, ni, nj)
		}
	}
}

func numIslandsDFSInMemory(grid [][]byte) int {
	if nil == grid || len(grid) == 0 {
		return 0
	}
	directions := [][]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	rows := len(grid)
	columns := len(grid[0])

	numOfIslands := 0
	for i := 0; i < rows; i++ {
		for j := 0; j < columns; j++ {
			if grid[i][j] == '1' {
				numOfIslands += 1
				dfsInMemory(grid, directions, i, j)
			}
		}
	}

	return numOfIslands
}
