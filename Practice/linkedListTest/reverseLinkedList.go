package main

import "fmt"
import linkedList "medhavat/algorithms/linkedList"

type Node = linkedList.Node

func main() {
	var node *Node
	for i := 5; i > 0; i-- {
		linkedList.Push(&node, i)
	}

	fmt.Println(node)
	fmt.Printf("\nLength of linked List %d", node.Length())
	revNode := linkedList.Reverse(node)
	fmt.Println("\nReverse node")
	fmt.Println(revNode)

	revNode = linkedList.Reverse(nil)
	fmt.Println("\nReverse node")
	fmt.Println(revNode)

	revNode = linkedList.Reverse(&Node{Val: 1})
	fmt.Println("\nReverse node")
	fmt.Println(revNode)
}
