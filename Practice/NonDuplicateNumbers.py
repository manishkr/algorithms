import unittest

class TestNonDuplicate(unittest.TestCase):
    def testCase0(self):
        nums = [2, 3, 3, 2, 5, 7, 9, 9]
        self.assertEqual((5, 7), FindNonDuplicate(nums))
    
    def testCase1(self):
        nums = [3, 3, 2, 4, 11, 16, 2, 4, 4, 4]
        self.assertEqual((11, 16), FindNonDuplicate(nums))

    def testCase2(self):
        nums = [100, 100, 100, 100, 100, 100, 100, 0, 0, 0, 0, 0]
        self.assertEqual((0, 100), FindNonDuplicate(nums))

def FindNonDuplicate(numbers):
    xor = 0
    for elem in numbers:
        xor ^= elem

    setBit = xor & ~(xor -1)
    
    first, second = 0, 0

    for elem in numbers:
        if elem & setBit:
            first ^= elem
        else:
            second ^=elem

    return min(first, second), max(first, second)

if __name__ == '__main__':
    unittest.main()
