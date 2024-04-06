import heapq

class MedianFinder:
    def __init__(self):
        self.max_heap = []  # max heap to store elements less than or equal to median
        self.min_heap = []  # min heap to store elements greater than or equal to median

    def addNum(self, num):
        if not self.max_heap or num <= -self.max_heap[0]:
            heapq.heappush(self.max_heap, -num)  # push to max heap
        else:
            heapq.heappush(self.min_heap, num)  # push to min heap

        # Balance the heaps
        if len(self.max_heap) > len(self.min_heap) + 1:
            heapq.heappush(self.min_heap, -heapq.heappop(self.max_heap))
        elif len(self.min_heap) > len(self.max_heap):
            heapq.heappush(self.max_heap, -heapq.heappop(self.min_heap))

    def findMedian(self):
        if len(self.max_heap) == len(self.min_heap):
            return (-self.max_heap[0] + self.min_heap[0]) / 2.0
        else:
            return -self.max_heap[0] / 1.0

# Example usage:
median_finder = MedianFinder()
nums = [2, 1, 3, 5, 4]
for num in nums:
    median_finder.addNum(num)

print(median_finder.findMedian())  # Output: 3.0
