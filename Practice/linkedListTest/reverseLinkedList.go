package main

import "fmt"
import linkedList "medhavat/algorithms/linkedList"

type Node = linkedList.Node

func main() {
	node := &Node{Val: 1}
	tail := node
	for i := 2; i <= 5; i++ {
		linkedList.Push(&(tail.Next), i)
		tail = tail.Next
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
