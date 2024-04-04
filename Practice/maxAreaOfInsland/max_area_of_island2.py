def is_valid(grid, i, j, rows, cols):
   return not(i < 0 or i >= rows or j < 0 or j >= cols or grid[i][j] != 1)

def max_area_of_island(grid):
    if not grid:
        return 0
    rows, cols = len(grid), len(grid[0])
    max_area = 0
    def dfs(i, j):
        if is_valid(grid, i, j, rows, cols):
            return 0
        grid[i][j] = -1
        area =1
        area += dfs(i + 1, j)
        area += dfs(i - 1, j)
        area += dfs(i, j +1)
        area += dfs(i, j-1)
        return area

    for i in range(rows):
        for j in range(cols):
            if grid[i][j] == 1:
                max_area = max(max_area, dfs(i, j))
    return max_area
