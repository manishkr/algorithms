package main

import "fmt"

func findPeakElementHelper(startIndex int, endIndex int, nums []int) int {
	if startIndex == endIndex {
		return startIndex
	}
	h := int(uint(startIndex+endIndex) >> 1)
	if nums[h] > nums[h+1] {
		return findPeakElementHelper(startIndex, h, nums)
	}
	return findPeakElementHelper(h+1, endIndex, nums)

}

func findPeakElement(nums []int) int {
	return findPeakElementHelper(0, len(nums)-1, nums)
}

func main() {
	nums := []int{1, 2}
	fmt.Println(findPeakElement(nums))
}
