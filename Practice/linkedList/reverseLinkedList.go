package main

import "fmt"

type Node struct {
	Val  int
	Next *Node
}

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

func printNode(node *Node) {
	temp := node
	for temp != nil {
		fmt.Print(temp.Val, "->")
		temp = temp.Next
	}
}
func main() {
	node := Node{Val: 1}
	node.Next = &Node{Val: 2}
	node.Next.Next = &Node{Val: 3}
	node.Next.Next.Next = &Node{Val: 4}
	node.Next.Next.Next.Next = &Node{Val: 5}

	printNode(&node)
	revNode := reverse(&node)
	fmt.Println("\nReverse node")
	printNode(revNode)

	revNode = reverse(nil)
	fmt.Println("\nReverse node")
	printNode(revNode)

	revNode = reverse(&Node{Val: 1})
	fmt.Println("\nReverse node")
	printNode(revNode)
}
