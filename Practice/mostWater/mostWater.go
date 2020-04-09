package main

func max(x int, y int) int {
	if x > y {
		return x
	}
	return y
}

func min(x, y int) int {
	if x < y {
		return x
	}
	return y
}
func maxArea(height []int) int {
	area := 0
	for i := 0; i < len(height); i++ {
		for j := i + 1; j < len(height); j++ {
			area = max(area, (j-i)*min(height[i], height[j]))
		}
	}

	return area
}
