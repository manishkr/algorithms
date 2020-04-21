package main

import "fmt"
import linkedList "medhavat/algorithms/linkedList"

type Node = linkedList.Node

func reverse(node *Node) *Node {
	var revNode *Node = nil
	for node != nil {
		temp := node.Next
		node.Next = revNode
		revNode = node
		node = temp
	}

	return revNode
}

func main() {
	node := Node{Val: 1}
	node.Next = &Node{Val: 2}
	node.Next.Next = &Node{Val: 3}
	node.Next.Next.Next = &Node{Val: 4}
	node.Next.Next.Next.Next = &Node{Val: 5}

	node.Print()
	revNode := reverse(&node)
	fmt.Println("\nReverse node")
	revNode.Print()

	revNode = reverse(nil)
	fmt.Println("\nReverse node")
	revNode.Print()

	revNode = reverse(&Node{Val: 1})
	fmt.Println("\nReverse node")
	revNode.Print()
}
