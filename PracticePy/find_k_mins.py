import heapq

class MaxHeap:
    def __init__(self):
        self.heap = []

    def parent(self, i):
        return (i - 1) // 2

    def left_child(self, i):
        return 2 * i + 1

    def right_child(self, i):
        return 2 * i + 2

    def insert(self, value):
        self.heap.append(value)
        self.heapify_up(len(self.heap) - 1)

    def heapify_up(self, i):
        while i > 0 and self.heap[self.parent(i)] < self.heap[i]:
            self.heap[self.parent(i)], self.heap[i] = self.heap[i], self.heap[self.parent(i)]
            i = self.parent(i)

    def extract_max(self):
        if len(self.heap) == 0:
            return None

        min_val = self.heap[0]
        self.heap[0] = self.heap[-1]
        self.heap.pop()
        self.heapify_down(0)
        return min_val

    def heapify_down(self, i):
        max_index = i
        left = self.left_child(i)
        right = self.right_child(i)

        if left < len(self.heap) and self.heap[left] > self.heap[max_index]:
            max_index = left

        if right < len(self.heap) and self.heap[right] > self.heap[max_index]:
            max_index = right

        if max_index != i:
            self.heap[i], self.heap[max_index] = self.heap[max_index], self.heap[i]
            self.heapify_down(max_index)

def find_k_mins(l, k):
    if len(l) <= k:
       return l

    result = l[0:k]
    heapq._heapify_max(result)
    print(result)
    for item in l[k:len(l)]:
        if item <= result[k -1]:

    heapq.heapify(result)
    return result


if __name__ == '__main__':
    l = [11, 3, 2, 14, 1, 2, 7]
    result = find_k_mins(l, 3)
    print("result", result)
