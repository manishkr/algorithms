package main

import (
	"fmt"
	"math/rand"
	"time"
)

func insertionSort(numbers []int) {
	size := len(numbers)
	for i := 0; i < size; i++ {
		for j := i; j < size; j++ {
			if numbers[i] > numbers[j] {
				tmp := numbers[i]
				numbers[i] = numbers[j]
				numbers[j] = tmp
			}
		}
	}
}

func main() {
	numbers := [10]int{5, 3, 6, 4, 1, 2, 1, 2, 1, 5}
	rand.Seed(time.Now().UnixNano())
	rand.Shuffle(len(numbers), func(i, j int) { numbers[i], numbers[j] = numbers[j], numbers[i] })
	fmt.Println(numbers)
	insertionSort(numbers[:])
	fmt.Println(numbers)
}
