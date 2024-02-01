package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	result := &ListNode{0, nil}
	lResult := result
	var quotient = 0

	for l1 != nil || l2 != nil {
		var val1, val2 = 0, 0
		if l1 != nil {
			val1 = l1.Val
		}
		if l2 != nil {
			val2 = l2.Val
		}

		sum := val1 + val2 + quotient

		quotient = sum / 10
		lResult.Next = &ListNode{sum % 10, nil}
		lResult = lResult.Next

		if l1 != nil {
			l1 = l1.Next
		}

		if l2 != nil {
			l2 = l2.Next
		}
	}

	if quotient > 0 {
		lResult.Next = &ListNode{quotient, nil}
	}
	return result.Next
}
func main() {
	l1 := ListNode{2, nil}
	l1.Next = &ListNode{4, nil}
	l1.Next.Next = &ListNode{6, nil}

	l2 := ListNode{5, nil}
	l2.Next = &ListNode{6, nil}
	l2.Next.Next = &ListNode{4, nil}

	addTwoNumbers(&l1, &l2)
}
