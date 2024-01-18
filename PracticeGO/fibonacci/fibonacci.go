package main

import (
	"fmt"
)

// fibonacci is a function that returns
// a function that returns an int.
func fibonacci() func() int64 {
	a, b := int64(0), int64(1)
	return func() int64 {
		x := a
		a = a + b
		b = x
		return b
	}
}

func main() {
	f := fibonacci()
	for i := 0; i < 100; i++ {
		fmt.Println(f())
	}
}
