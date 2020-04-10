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
	trapArea := 0
	for i := 0; i < len(height); i++ {
		leftMax, rightMax := 0, 0
		for j := 0; j < i; j++ {
			leftMax = max(leftMax, height[j])
		}
		for j := i + 1; j < len(height); j++ {
			rightMax = max(rightMax, height[j])
		}
		area := min(leftMax, rightMax) - height[i]
		if area > 0 {
			trapArea += area
		}
	}

	return trapArea
}
