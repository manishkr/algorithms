package main

import "fmt"

func peakIndexInMountainHelper(startIndex int, endIndex int, nums []int) int {
	if startIndex == endIndex {
		return startIndex
	}
	h := int(uint(startIndex+endIndex) >> 1)
	if nums[h] > nums[h+1] {
		return peakIndexInMountainHelper(startIndex, h, nums)
	}
	return peakIndexInMountainHelper(h+1, endIndex, nums)

}

func peakIndexInMountainArray(nums []int) int {
	return peakIndexInMountainHelper(0, len(nums)-1, nums)
}

func main() {
	nums := []int{0, 2, 1, 0}
	fmt.Println(peakIndexInMountainArray(nums))
}
