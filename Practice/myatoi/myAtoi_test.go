package main

import (
	"math"
	"testing"
)

func TestMyAtoi(t *testing.T) {
	input := "42"
	output := myAtoi(input)
	if output != 42 {
		t.Errorf("myAtoi(input) = %d; want 42", output)
	}

	input = "   -42"
	output = myAtoi(input)
	if output != -42 {
		t.Errorf("myAtoi(input) = %d; want -42", output)
	}

	input = "4193 with words"
	output = myAtoi(input)
	if output != 4193 {
		t.Errorf("myAtoi(input) = %d; want 4193", output)
	}

	input = "words and 987"
	output = myAtoi(input)
	if output != 0 {
		t.Errorf("myAtoi(input) = %d; want 0", output)
	}

	input = "-91283472332"
	output = myAtoi(input)
	if output != math.MinInt32 {
		t.Errorf("myAtoi(input) = %d; want %d", output, math.MinInt32)
	}
}
