package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func Print(listNode *ListNode) {
	for listNode != nil {
		fmt.Print(listNode.Val, "->")
		listNode = listNode.Next
	}
	fmt.Println("")
}

func Reverse(listNode *ListNode) *ListNode {
	var reverseNode *ListNode = nil
	for listNode != nil {
		tmpNode := listNode.Next
		listNode.Next = reverseNode
		reverseNode = listNode
		listNode = tmpNode
	}
	return reverseNode
}
func main() {
	node := ListNode{
		2, nil,
	}
	node.Next = &ListNode{3, nil}
	node.Next.Next = &ListNode{4, nil}
	node.Next.Next.Next = &ListNode{5, nil}
	node.Next.Next.Next.Next = &ListNode{6, nil}
	Print(&node)
	reverseNode := Reverse(&node)
	Print(reverseNode)
}
