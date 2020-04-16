package main

import (
	"fmt"
	"testing"
)

func TestMovieCombination(t *testing.T) {
	movieTimes := []Movie{{1, 10}, {2, 10},
		{3, 15}, {4, 20}, {5, 100},
		{6, 20}, {7, 100}, {8, 100}, {9, 20},
		{10, 20}, {11, 20}}
	matchedMovies := findMovieCombination(movieTimes, 100)
	for _, matchedMovie := range matchedMovies {
		fmt.Println(matchedMovie)
	}
}
