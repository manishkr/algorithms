package main

func max(x int, y int) int {
	if x > y {
		return x
	}

	return y
}
func min(x int, y int) int {
	if x < y {
		return x
	}

	return y
}
func trap(height []int) int {
	size := len(height)
	if size == 0 {
		return 0
	}
	trapArea := 0
	left := make([]int, len(height))
	right := make([]int, len(height))
	left[0] = height[0]
	right[size-1] = height[size-1]

	for i := 1; i < len(height); i++ {
		left[i] = max(height[i], left[i-1])
	}
	for i := size - 2; i > 0; i-- {
		right[i] = max(height[i], right[i+1])
	}

	for i := 0; i < size; i++ {
		area := min(left[i], right[i]) - height[i]
		if area > 0 {
			trapArea += area
		}
	}

	return trapArea
}
