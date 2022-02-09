#!/usr/bin/python3.4
import unittest
import random

class RedixSortTest(unittest.TestCase):
    def doTest(self, inList):
        #Check with default sort
        self.assertEqual(sorted(inList), QuickSort(inList))

    def testCase0(self):
        self.doTest([])

    def testCase1(self):
        self.doTest([1])

    def testCase2(self):
        self.doTest([1, 2, 3, 4, 5, 6, 7, 8])

    def testCase3(self):
        inList = [234, 334, 34, 346, 11,0, 121, 1211, 9999, 99999999, 89761, 678, 100]
        self.doTest(inList)

    def testCase4(self):
        inList = [1, 3, 121, 34, 12, 12, 2411]
        self.doTest(inList)

    def testCase5(self):
        inList = [1, 11, 111, 1111, 11111]
        self.doTest(inList)
    
    def testCase6(self):
        inList = [3333, 333, 33, 3]
        self.doTest(inList)

    def testCase7(self):
        random.seed(100)

        inList = []
        for i in range(10):
            base = random.randint(1, 10)
            num1 = base**random.randint(0, 5)
            num2 = base**random.randint(6, 10)
            inList.extend([random.randint(num1, num2) for x in range(i*10)])
            random.shuffle(inList)
        self.doTest(inList)

    def testCase8(self):
        self.doTest([0, 0, 0, 0, 0, 0])

    def testCase9(self):
        self.doTest([0, 000, 1234, 4321, 121, 10101, 1, 101, 221, 10000, 12344, 12345, 54321])

def QuickSort(inList):
    if not inList:
        return []

    copyList = inList[:]

    DoQuickSort(copyList, 0, len(copyList) -1)
    return copyList

def DoQuickSort(inList, stIndex, endIndex):
    if stIndex < endIndex:
        index = Partition(inList, stIndex, endIndex)
        DoQuickSort(inList, stIndex, index -1)
        DoQuickSort(inList, index + 1, endIndex)

def Swap(inList, index1, index2):
    tmp = inList[index1]
    inList[index1] = inList[index2]
    inList[index2] = tmp

def Partition(inList, stIndex, endIndex):
    elem = inList[endIndex]
    p = stIndex - 1
    for i in range(stIndex, endIndex):
        if inList[i] <= elem:
            p +=1
            Swap(inList, p, i)

    Swap(inList, p+1, endIndex)

    return p + 1

if __name__ == '__main__':
    unittest.main()
