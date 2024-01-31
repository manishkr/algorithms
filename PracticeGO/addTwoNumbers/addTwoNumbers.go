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

func Add(l1 *ListNode, l2 *ListNode) *ListNode {
	sumNode := &ListNode{0, nil}
	result := sumNode
	cf := 0
	for l1 != nil || l2 != nil {
		val := cf
		if l1 != nil {
			val += l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			val += l2.Val
			l2 = l2.Next
		}
		cf = val / 10
		result.Next = &ListNode{val % 10, nil}
		result = result.Next
	}

	if cf > 0 {
		result.Next = &ListNode{cf, nil}
	}
	return sumNode.Next
}

func main() {
	node1 := ListNode{
		2, nil,
	}
	node1.Next = &ListNode{3, nil}
	node1.Next.Next = &ListNode{4, nil}

	node2 := ListNode{
		3, nil,
	}
	node2.Next = &ListNode{5, nil}
	Print(&node1)
	Print(&node2)
	result := Add(&node1, &node2)
	Print(result)
}
