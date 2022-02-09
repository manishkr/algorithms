import unittest

class TestDuplicateNumbers(unittest.TestCase):
    def testCase0(self):
        nums = [1, 2, 1, 2, 3, 4, 5, 5]
        self.assertEqual([1, 2, 5], sorted(DuplicateNumbers(nums)))

    def testCase1(self):
        nums = [2, 2, 3, 3, 2, 2, 2, 2, 3, 3, 5, 5, 1, 10, 111, 234, 234]
        self.assertEqual([2, 3, 5, 234], sorted(DuplicateNumbers(nums)))

def DuplicateNumbers(nums):
    numMap = {}
    for elem in nums:
        if elem in numMap:
            numMap[elem] += 1
        else:
            numMap[elem] = 1

    dupList = []
    for key in numMap:
        if numMap[key] > 1:
            dupList.append(key)

    return dupList

if __name__ == '__main__':
    unittest.main()
