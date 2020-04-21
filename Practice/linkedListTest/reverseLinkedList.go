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

	node.Print()
	revNode := linkedList.Reverse(&node)
	fmt.Println("\nReverse node")
	revNode.Print()

	revNode = linkedList.Reverse(nil)
	fmt.Println("\nReverse node")
	revNode.Print()

	revNode = linkedList.Reverse(&Node{Val: 1})
	fmt.Println("\nReverse node")
	revNode.Print()
}
