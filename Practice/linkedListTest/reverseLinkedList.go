package main

import "fmt"
import linkedList "medhavat/algorithms/linkedList"

type Node = linkedList.Node

func main() {
	node := Node{Val: 1}
	node.Next = &Node{Val: 2}
	node.Next.Next = &Node{Val: 3}
	node.Next.Next.Next = &Node{Val: 4}
	node.Next.Next.Next.Next = &Node{Val: 5}
	fmt.Println(&node)
	fmt.Printf("\nLength of linked List %d", node.Length())
	revNode := linkedList.Reverse(&node)
	fmt.Println("\nReverse node")
	fmt.Println(revNode)

	revNode = linkedList.Reverse(nil)
	fmt.Println("\nReverse node")
	fmt.Println(revNode)

	revNode = linkedList.Reverse(&Node{Val: 1})
	fmt.Println("\nReverse node")
	fmt.Println(revNode)
}
