package algorithms

import "fmt"

type Node struct {
	Val  int
	Next *Node
}

func (node *Node) Print() {
	temp := node
	for temp != nil {
		fmt.Print(temp.Val, "->")
		temp = temp.Next
	}
}
