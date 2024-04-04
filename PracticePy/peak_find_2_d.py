def find_peak_2d(matrix):
    rows, cols = len(matrix), len(matrix[0])

    def find_max_in_column(col):
        max_val = float('-inf')
        max_row = -1
        for i in range(rows):
            if matrix[i][col] > max_val:
                max_val = matrix[i][col]
                max_row = i
        return max_row

    def is_peak(row, col):
        current = matrix[row][col]
        top = matrix[row - 1][col] if row > 0 else float('-inf')
        bottom = matrix[row + 1][col] if row < rows - 1 else float('-inf')
        left = matrix[row][col - 1] if col > 0 else float('-inf')
        right = matrix[row][col + 1] if col < cols - 1 else float('-inf')
        return current > top and current > bottom and current > left and current > right

    def search_peak(start_col, end_col):
        if start_col == end_col:
            return (find_max_in_column(start_col), start_col)

        mid_col = (start_col + end_col) // 2
        max_row = find_max_in_column(mid_col)

        if is_peak(max_row, mid_col):
            return (max_row, mid_col)
        elif matrix[max_row][mid_col - 1] > matrix[max_row][mid_col]:
            return search_peak(start_col, mid_col - 1)
        else:
            return search_peak(mid_col + 1, end_col)

    return search_peak(0, cols - 1)

if __name__ == '__main__':
    l =[[10,50,40,30,20],[1,500,2,3,4]]
    result = find_peak_2d(l)
    print(result)
