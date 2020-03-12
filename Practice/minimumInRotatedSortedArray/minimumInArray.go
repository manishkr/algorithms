package main

func findMin(nums []int) int {
	if len(nums) == 1 {
		return nums[0]
	}

	i, j := 0, len(nums)-1
	if nums[j] > nums[i] {
		return nums[i]
	}

	for i <= j {
		h := int(uint(i+j) >> 1)
		if nums[h] > nums[h+1] {
			return nums[h+1]
		}
		if nums[h-1] > nums[h] {
			return nums[h]
		}

		if nums[h] > nums[0] {
			i = h + 1
		} else {
			j = h - 1
		}

	}

	return nums[i]
}
