#!/usr/bin/python3

import unittest

class LongestPalindromeTest(unittest.TestCase):
    def testCase1(self):
        self.assertEqual("carac", FindLongestPlaindromeMeomoize("character"))

    def testCase2(self):
        self.assertEqual("pcabacp", FindLongestPlaindromeMeomoize("pcqaqbacp"))

    def testCase3(self):
        self.assertEqual("abcba", FindLongestPlaindromeMeomoize("abcba"))
 
    def testCase4(self):
        self.assertEqual("aaaaaa", FindLongestPlaindromeMeomoize("aaaaaa"))

    def testCase5(self):
        self.assertEqual("a", FindLongestPlaindromeMeomoize("a"))
    
    def testCase6(self):
        self.assertEqual("", FindLongestPlaindromeMeomoize(""))

def FindLongestPlaindromeMeomoize(str):
    table = CreateLengthTable(str)
    size = len(str)
    slen = 0

    if size > 0:
        slen = table[0][size-1]
    
    subseq = ['']*slen
    i, k = 0, 0
    j = size - 1 
    while i <= j :
        if str[i] == str[j]:
            subseq[k]= str[i]
            subseq[slen-k-1] = str[i]
            i = i + 1
            j = j - 1
            k = k + 1
        elif table[i][j-1] > table[i+1][j]:
            j = j - 1
        elif i == j:
            subseq[k] = str[i]
        else:
            i = i + 1
    
    return ''.join(subseq)
        

def CreateLengthTable(str):
    size = len(str)
    table = [[0 for x in range(size)] for x in range(size)]
   
    for i in range(size):
        table[i][i] = 1

    for i in range(2, size+1):
        for j in range(size - i + 1):
            k = j + i - 1
            if str[j] == str[k] and i == 2:
                table[j][k] = 2
            elif str[j] == str[k]:
                table[j][k] = table[j+1][k-1] + 2
            else:
                table[j][k] = max(table[j][k-1], table[j+1][k])
    
    return table

if __name__ == '__main__':
    unittest.main()
