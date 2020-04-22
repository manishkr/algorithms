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

func (node *Node) Length() int {
	current := node
	count := 0
	for current != nil {
		count += 1
		current = current.Next
	}

	return count
}

func Reverse(node *Node) *Node {
	var revNode *Node = nil
	for node != nil {
		temp := node.Next
		node.Next = revNode
		revNode = node
		node = temp
	}

	return revNode
}
