package main

import "fmt"

func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	shortestArray := nums1
	longestArray := nums2
	if len(nums1) > len(nums2) {
		shortestArray = nums2
		longestArray = nums1
	}
	min := 0
	max := len(shortestArray)
	halfLen := (len(nums1) + len(nums2) + 1) / 2

	for min <= max {
		i := (min + max) / 2
		j := halfLen - i
		if i < max && longestArray[j-1] > shortestArray[i] {
			min = i + 1
		} else if i > min && shortestArray[i-1] > longestArray[j] {
			max = i - 1
		} else {
			maxLeft := 0
			if i == 0 {
				maxLeft = longestArray[j-1]
			} else if j == 0 {
				maxLeft = shortestArray[i-1]
			} else {
				if shortestArray[i-1] > longestArray[j-1] {
					maxLeft = shortestArray[i-1]
				} else {
					maxLeft = longestArray[j-1]
				}
			}
			if (len(nums1)+len(nums2))%2 == 1 {
				return float64(maxLeft)
			}

			minRight := 0
			if i == len(shortestArray) {
				minRight = longestArray[j]
			} else if j == len(longestArray) {
				minRight = shortestArray[i]
			} else {
				if shortestArray[i] < longestArray[j] {
					minRight = shortestArray[i]
				} else {
					minRight = longestArray[j]
				}
			}
			return float64(maxLeft+minRight) / 2.0
		}
	}
	return 0.0
}

func main() {
	nums1 := []int{1, 3}
	nums2 := []int{2}
	fmt.Println(findMedianSortedArrays(nums1, nums2))
	nums1 = []int{1, 2}
	nums2 = []int{3, 4}
	fmt.Println(findMedianSortedArrays(nums1, nums2))
}
