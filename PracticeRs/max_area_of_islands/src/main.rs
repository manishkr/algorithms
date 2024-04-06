fn is_valid(i: usize, j: usize, grid: &Vec<Vec<i32>>, visited: &Vec<Vec<bool>>) -> bool {
    if i >= grid.len() || j >= grid[0].len() || grid[i][j] == 0 || visited[i][j] == true {
        return false;
    }
    true
}

fn dfs(i: usize, j: usize, grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> i32 {
    let mut area = 0;
    area += 1;
    if !is_valid(i, j, grid, visited) {
        return 0;
    }
    visited[i][j] = true;
    if i >= 1 {
        area += dfs(i - 1, j, grid, visited);
    }
    area += dfs(i + 1, j, grid, visited);
    if j >= 1 {
        area += dfs(i, j - 1, grid, visited);
    }
    area += dfs(i, j + 1, grid, visited);
    area
}

fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    if grid.len() == 0 || grid[0].len() == 0 {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut max_area = 0;
    let mut visited: Vec<Vec<bool>> = Vec::new();
    for i in 0..rows {
        visited.push(Vec::new());
        for _ in 0..cols {
            visited[i].push(false);
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 1 {
                max_area = i32::max(max_area, dfs(i, j, &grid, &mut visited))
            }
        }
    }
    max_area
}
fn main() {
    let grid = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    ];
    let result = max_area_of_island(grid);
    println!("{}", result);

    let grid = vec![vec![1]];
    let result = max_area_of_island(grid);
    println!("{}", result);
}
