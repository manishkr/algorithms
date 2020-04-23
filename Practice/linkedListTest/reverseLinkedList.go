package main

import "fmt"
import linkedList "medhavat/algorithms/linkedList"

type Node = linkedList.Node

func main() {
	node := &Node{Val: 5}
	linkedList.Push(&node, 4)
	linkedList.Push(&node, 3)
	linkedList.Push(&node, 2)
	linkedList.Push(&node, 1)

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
