package algorithms

import (
	"strconv"
)

type Node struct {
	Val  int
	Next *Node
}

func (node *Node) String() string {
	str := ""
	for current := node; current != nil; {
		str = str + strconv.Itoa(current.Val)
		current = current.Next
		if current != nil {
			str = str + "->"
		}
	}

	return str
}

func (node *Node) Length() int {
	count := 0
	for current := node; current != nil; current = current.Next {
		count += 1
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

func Push(node **Node, data int) {
	newNode := Node{Val: data}
	newNode.Next = *node
	*node = &newNode
}
