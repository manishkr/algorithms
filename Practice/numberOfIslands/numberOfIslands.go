package main

type Pair struct {
	i, j int
}

func isValid(grid [][]byte, visited [][]bool, i int, j int) bool {
	if i < 0 || i >= len(grid) || j < 0 || j >= len(grid[0]) || visited[i][j] || grid[i][j] == '0' {
		return false
	}

	return true
}

func bfs(grid [][]byte, visited [][]bool, directions [][]int, i int, j int) {
	queue := []Pair{{i, j}}
	for len(queue) > 0 {
		pair := queue[len(queue)-1]
		queue = queue[:len(queue)-1]
		for index := 0; index < len(directions); index++ {
			ni := pair.i + directions[index][0]
			nj := pair.j + directions[index][1]

			if isValid(grid, visited, ni, nj) {
				queue = append(queue, Pair{ni, nj})
				visited[ni][nj] = true
			}
		}
	}
}
func numIslands(grid [][]byte) int {
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
				bfs(grid, visited, directions, i, j)
			}
		}
	}

	return numOfIslands
}
