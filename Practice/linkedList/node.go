package algorithms

import (
	"strconv"
)

type Node struct {
	Val  int
	Next *Node
}

func (node *Node) String() string {
	temp := node
	str := ""
	for temp != nil {
		str = str + strconv.Itoa(temp.Val)
		temp = temp.Next
		if temp != nil {
			str = str + "->"
		}
	}

	return str
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
