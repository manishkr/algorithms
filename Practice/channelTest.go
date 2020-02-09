package main

import (
	"fmt"
	"time"
)

func readword(ch chan string) {
	fmt.Println("Type a word, then hit Enter.")
	var word string
	fmt.Scanf("%s", &word)
	ch <- word
}

func timeout(t chan bool) {
	time.Sleep(10 * time.Second)
	t <- true
}

func timeout2(t chan bool) {
	time.Sleep(20 * time.Second)
	t <- true
}

func main() {
	t := make(chan bool)
	go timeout(t)

	t1 := make(chan bool)
	go timeout2(t1)

	s1, s2 := <-t, <-t1
	print(s1, s2)
}
