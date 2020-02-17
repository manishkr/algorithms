package main

func subarraySum(nums []int, k int) int {
	count, sum := 0, 0
	sumMap := make(map[int]int)
	sumMap[0] = 1
	for i := 0; i < len(nums); i++ {
		sum += nums[i]
		value, ok := sumMap[sum-k]
		if ok {
			count += value
		}
		value1, ok := sumMap[sum]
		if ok {
			sumMap[sum] = value1 + 1
		} else {
			sumMap[sum] = 1
		}
	}

	return count
}
