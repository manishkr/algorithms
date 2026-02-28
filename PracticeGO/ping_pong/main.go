package main

import (
	"fmt"
	"sync"
	"time"
)

// Ball represents the signal that is passed between ping and pong goroutines.
type Ball struct {
	hits int
}

func main() {
	var wg sync.WaitGroup

	// Create channels for communication between ping and pong
	pingChannel := make(chan *Ball)
	pongChannel := make(chan *Ball)

	// Start the ping and pong goroutines
	wg.Add(2)
	go ping("Player 1", pingChannel, pongChannel, &wg)
	go ping("Player 2", pongChannel, pingChannel, &wg)

	// Start the game by sending the first ball to the ping goroutine
	pingChannel <- &Ball{}

	// Wait for the goroutines to finish
	wg.Wait()
}

func ping(name string, in <-chan *Ball, out chan<- *Ball, wg *sync.WaitGroup) {
	defer wg.Done()

	for {
		ball := <-in
		ball.hits++
		fmt.Printf("%s hits the ball %d times\n", name, ball.hits)
		time.Sleep(time.Millisecond * 500) // Simulate some processing time

		out <- ball
	}
}
