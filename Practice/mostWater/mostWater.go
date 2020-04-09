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
	l, r := 0, len(height)-1
	for l < r {
		area = max(area, (r-l)*min(height[l], height[r]))
		if height[l] < height[r] {
			l++
		} else {
			r--
		}
	}

	return area
}
