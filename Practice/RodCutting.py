#!/usr/bin/python3
import sys
import unittest

class RodCuttingTest(unittest.TestCase):
    def setUp(self):
        self.PriceTable = {1:1, 2:5, 3:8, 4:9, 5:10, 6:17, 7:17, 8:20, 9:24, 10:30}

    def testCase1(self):
        self.assertEqual(10, CutRod(self.PriceTable, 4))
        self.assertEqual(10, MemoizedCutRod(self.PriceTable, 4))
        self.assertEqual(10, BottomUpCutRod(self.PriceTable, 4))

    def testCase2(self):
        self.assertEqual(0, CutRod(self.PriceTable, 0))
        self.assertEqual(0, MemoizedCutRod(self.PriceTable, 0))
        self.assertEqual(0, BottomUpCutRod(self.PriceTable, 0))

    def testCase3(self):
        self.assertEqual(22, CutRod(self.PriceTable, 8))
        self.assertEqual(22, MemoizedCutRod(self.PriceTable, 8))
        self.assertEqual(22, BottomUpCutRod(self.PriceTable, 8))

def BottomUpCutRod(priceTable, rodSize):
    minVal = -sys.maxsize -1
    values = [0]*(rodSize + 1)

    for i in range(1, rodSize + 1):
        rodValue = minVal

        for j in range(1, i + 1):
            rodValue = max(rodValue, priceTable[j] + values[i - j])

        values[i] = rodValue

    return values[rodSize]

def MemoizedCutRod(priceTable, rodSize):
    minVal = -sys.maxsize - 1
    values = [minVal]*(rodSize + 1)

    return MemoizedCutRodAux(priceTable, rodSize, values)

def MemoizedCutRodAux(priceTable, rodSize, values):
    rodValue = 0
    if values[rodSize] >= 0:
        rodValue = values[rodSize]
    elif rodSize > 0:
        rodValue = -sys.maxsize - 1
        for i in range(1, rodSize + 1):
            rodValue = max(rodValue, priceTable[i] + MemoizedCutRodAux(priceTable, rodSize - i, values))

    values[rodSize] = rodValue

    return rodValue

def CutRod(priceTable, rodSize):
    rodValue = 0
    if rodSize > 0:
        rodValue = -sys.maxsize - 1

        for i in range(1, rodSize + 1):
            rodValue = max(rodValue, priceTable[i] + CutRod(priceTable, rodSize - i))

    return rodValue

if __name__ == '__main__':
    unittest.main()
