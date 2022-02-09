#!/usr/bin/python3.4
'''
Radix sort:
Assumption: list contains only non negative integers
[June 17, 2015] : Added RadixSortForAll to handle negative number also!!
'''
import unittest
import random

class RedixSortTest(unittest.TestCase):
    def doTest(self, inList):
        #Check with default sort
        self.assertEqual(sorted(inList), RadixSort(inList))

    def doAllTest(self, inList):
        #Check with default sort
        self.assertEqual(sorted(inList), RadixSortForAll(inList))

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

    def testCase10(self):
        self.doAllTest([100, -1, -121, -75, 121, 12132, 1312, -1121])

    def testCase11(self):
        random.seed(100)

        inList = []
        for i in range(10):
            base = random.randint(1, 10)
            num1 = base**random.randint(1, 5)
            num2 = base**random.randint(6, 10)
            inList.extend([((-1)**base)*random.randint(num1, num2) for x in range(i*10)])
            random.shuffle(inList)
        self.doAllTest(inList)

def RadixSortForAll(inList):
    negList = [-1*x for x in inList if x < 0]
    negMainBin = RadixSort(negList)
    negMainBin = reversed(negMainBin)
    negMainBin = [-1*x for x in negMainBin]
    
    nonNegList = list(filter(lambda x : x >= 0, inList))
    nonNegMainBin = RadixSort(nonNegList)

    mainBin = []
    mainBin.extend(negMainBin)
    mainBin.extend(nonNegMainBin)
    
    return mainBin

def RadixSort(inList):
    if not inList:
        return []

    maxValue = max(inList)
    numDigits = len(str(maxValue))
    
    digitBin = {}
    mainBin = inList[:]
    
    for i in range(1, numDigits+1):
        currentMod = 10**i
        currentDiv = currentMod/10
        for elem in mainBin:
            digit = (elem % currentMod)//currentDiv
            
            if digit not in digitBin.keys():
                digitBin[digit] = []
            digitBin[digit].append(elem)

        mainBin.clear()
        for i in range(10):
            if i in digitBin.keys():
                mainBin.extend(digitBin[i])
        digitBin.clear()

    return mainBin

if __name__ == '__main__':
    unittest.main()
