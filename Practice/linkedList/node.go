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
