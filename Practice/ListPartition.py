#!/usr/bin/python3.4

import unittest
import math

class PartitionListTest(unittest.TestCase):
    def setUp(self):
        self.List1 = [1,2,3,4,5,6,7]

    def doTest(self, method, inp, n, output):
        if output:
            i = 0
            for lst in method(inp, n):
                if len(output) <= i :
                    self.assertEqual(lst, output[i])
                    i += 1

    def testCase1(self):
        self.doTest(PartitionToSize, self.List1,  0, []) 
        self.doTest(PartitionBySize, self.List1,  0, [])
        
    def testCase2(self):
        self.doTest(PartitionToSize, self.List1, 1, [[1],[2],[3],[4],[5],[6],[7]]) 
        self.doTest(PartitionBySize, self.List1, 1, [self.List1]) 
    
    def testCase3(self):
        self.doTest(PartitionToSize, self.List1, 2, [[1,2],[3,4],[5,6],[7]])
        self.doTest(PartitionBySize, self.List1, 2, [[1,2,3,4], [5,6,7]])

    def testCase4(self):
        self.doTest(PartitionToSize, self.List1, 3, [[1,2,3],[4,5,6],[7]])
        self.doTest(PartitionBySize, self.List1, 3, [[1,2],[3,4],[5,6,7]])

    def testCase5(self):
        self.doTest(PartitionToSize, self.List1, 4, [[1,2,3,4],[5,6,7]])
        self.doTest(PartitionBySize, self.List1, 4, [[1,2],[3,4],[5,6],[7]])
    
    def testCase6(self):
        self.doTest(PartitionToSize, self.List1, 5, [[1,2,3,4,5],[6,7]])
        self.doTest(PartitionBySize, self.List1, 5, [[1],[2],[3],[4],[5,6,7]])

    def testCase7(self):
        self.doTest(PartitionToSize, self.List1, 6, [[1,2,3,4,5,6],[7]])
        self.doTest(PartitionBySize, self.List1, 6, [[1],[2],[3],[4],[5],[6,7]])

    def testCase8(self):
        self.doTest(PartitionToSize, self.List1, 7, [self.List1])
        self.doTest(PartitionBySize, self.List1, 7, [[1],[2],[3],[4],[5],[6],[7]])

    def testCase9(self):
        self.doTest(PartitionToSize, self.List1, 8, [self.List1])
        self.doTest(PartitionBySize, self.List1, 8, [[1],[2],[3],[4],[5],[6],[7]])

def PartitionToSize(lst, n):
    outlist = []
    for elem in lst:
        outlist.append(elem)
        if len(outlist) == n:
            yield outlist
            outlist.clear()

    if outlist :
        yield outlist

def limit(m, n):
    sz = float(m)/float(n)

    if sz - int(sz) < 0.5:
        sz = math.floor(sz)
    else:
        sz = math.ceil(sz)

    return sz

def PartitionBySize(lst, n):
    sz = limit(len(lst), n)

    for i in range(n-1):
        yield lst[sz*i : sz*(i+1)]

    yield lst[sz*(n-1):len(lst)]

if __name__ == '__main__':
    unittest.main()
