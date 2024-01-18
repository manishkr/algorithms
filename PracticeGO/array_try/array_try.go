package main

import (
	"fmt"
	"strings"
)

func main() {
	var a [2]string
	a[0] = "Hello"
	a[1] = "World"
	fmt.Println(a[0], a[1])
	fmt.Println(a)

	primes := [6]int{2, 3, 5, 7, 11, 13}
	fmt.Println(primes)
	var s []int = primes[1:4]
	fmt.Println(s)
	s[2] = 15
	fmt.Println(s)
	fmt.Println(primes)
	primes[2] = 17
	fmt.Println(s)
	fmt.Println(primes)

	x := []bool{true, true, false}
	fmt.Println(x)
	y := append(x, true)
	fmt.Println(y)
	fmt.Println(x)

	t := []struct {
		i int
		b bool
	}{
		{2, true},
		{3, false},
		{5, true},
		{7, true},
		{11, false},
		{13, true},
	}
	fmt.Println(t)

	u := []int{2, 3, 5, 7, 11, 13}
	printSlice(u)

	// Slice the slice to give it zero length.
	u = u[:0]
	printSlice(u)

	// Extend its length.
	u = u[:4]
	printSlice(u)

	// Drop its first two values.
	u = u[2:]
	printSlice(u)

	var v []int
	fmt.Println(v, len(v), cap(v))
	if v == nil {
		fmt.Println("nil!")
	}

	e := make([]int, 5)
	printSlice2("e", e)

	b := make([]int, 0, 5)
	printSlice2("b", b)

	c := b[:2]
	printSlice2("c", c)

	d := c[2:5]
	printSlice2("d", d)

	board := [][]string{
		[]string{"_", "_", "_"},
		[]string{"_", "_", "_"},
		[]string{"_", "_", "_"},
	}

	// The players take turns.
	board[0][0] = "X"
	board[2][2] = "O"
	board[1][2] = "X"
	board[1][0] = "O"
	board[0][2] = "X"

	for i := 0; i < len(board); i++ {
		fmt.Printf("%s\n", strings.Join(board[i], " "))
	}

	pow := make([]int, 10)
	for i := range pow {
		pow[i] = 1 << uint(i) // == 2**i
	}
	for _, value := range pow {
		fmt.Printf("%d\n", value)
	}
}
func printSlice(s []int) {
	fmt.Printf("len=%d cap=%d %v\n", len(s), cap(s), s)
}

func printSlice2(s string, x []int) {
	fmt.Printf("%s len=%d cap=%d %v\n",
		s, len(x), cap(x), x)
}
