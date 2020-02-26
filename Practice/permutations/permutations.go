package main

func permuteHelper(nums []int, set *[]int, result *[][]int) {
	if len(nums) == 0 {
		*result = append(*result, [][]int{*set}...)
	}
	for i := 0; i < len(nums); i++ {
		elem := nums[i]
		x, y := nums[:i], nums[i+1:]
		leftArray := make([]int, len(x))
		copy(leftArray, x)
		leftArray = append(leftArray, y...)
		*set = append([]int{elem}, *set...)
		permuteHelper(leftArray, set, result)
		*set = (*set)[1:]
	}
}

func permute(nums []int) [][]int {
	result := [][]int{}
	permuteHelper(nums, &[]int{}, &result)

	return result
}
