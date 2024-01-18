package main

import (
	"fmt"
	"math"
	"math/rand"
)

var c, python, java bool

func split(sum int) (x, y int) {
	x = sum * 4 / 9
	y = sum - x
	return
}

func split_without_name(sum int) (int, int) {
	x := sum * 4 / 9
	y := sum - x
	return x, y
}

func add(a int64, b int64) int64 {
	return a + b

}
func main() {
	fmt.Println("A randrom number is: ", rand.Intn(10))
	fmt.Println("Square root of 4 is: ", math.Sqrt(4))
	fmt.Println("Square root of 7 is: ", math.Sqrt(7))
	fmt.Println("Ceil of 4.5 is: ", math.Ceil(4.5))
	fmt.Println("Floor of 4.5 is: ", math.Floor(4.5))
	fmt.Println("Pow of 2 and 3 is: ", math.Pow(2, 3))
	fmt.Println("Pi is: ", math.Pi)

	fmt.Println("Addition of 2 and 3 is: ", add(2, 3))

	fmt.Println(split(17))
	fmt.Println(split(20))
	fmt.Println(split(123))
	fmt.Println(split_without_name(17))

	var i = 11
	j := 12
	i = 1112
	fmt.Println(i, j, c, python, java)
}
