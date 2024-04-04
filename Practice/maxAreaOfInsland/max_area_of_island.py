def is_vaiid(grid, visited, i, j):
    return not (i < 0 or i >= len(grid) or j < 0 or j >= len(grid) or visited[i][j] or grid[i][j] == 0)

def dfs(grid, visited, directions, i, j):
   area = 0
   for index in range(0, len(directions)):
       ni = i + directions[i][0]
       nj = j + directions[j][1]

       if is_vaiid(grid, visited, ni, nj):
           visited[ni][nj] = True
           area += 1
           area += dfs(grid, visited, directions, ni, nj)

    return area

def max_area_of_island(grid):
    if not grid:
        return 0
    directions = [[1, 0], [-1, 0], [0, 1], [0, -1]]
    rows = len(grid)
    columns = len(grid[0])
    visited = [[False] * columns for _ in range(rows)]
    max_area_of_island = 0
    for i in range(0, rows):
        for j in range(0, columns):
            if grid[i][j] == 1 and not visited[i][j]
                visited[i][j] = True
                area = 1 + dfs(grid, visited, directions, i, j)
                max_area_of_island = max(max_area_of_island, area)
    return max_area_of_island
