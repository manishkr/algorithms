package main

func productExceptSelf(nums []int) []int {
	size := len(nums)
	if size == 0 {
		return []int{1}
	}

	output := make([]int, size)
	output[0] = 1
	for i := 1; i < size; i++ {
		output[i] = output[i-1] * nums[i-1]
	}

	r := 1
	for i := size - 1; i >= 0; i-- {
		output[i] = output[i] * r
		r *= nums[i]
	}
	return output
}
