#!/usr/bin/python3

import unittest

class LongestCommonSubSequenceTest(unittest.TestCase):
    def testCase1(self):
        self.assertEqual("cher", FindLongestCommonSubSequence("character", "chdldjlder"))

    def testCase2(self):
        self.assertEqual("pcqqp", FindLongestCommonSubSequence("pcqaqbacp", "pcqqqdddp"))

    def testCase3(self):
        self.assertEqual("abcdef", FindLongestCommonSubSequence("abcdef", "abcdef"))

    def testCase4(self):
        self.assertEqual("", FindLongestCommonSubSequence("abc", "def"))

    def testCase5(self):
        self.assertEqual("", FindLongestCommonSubSequence("", ""))

def FindLongestCommonSubSequence(str1, str2):
    table = CreateLengthTable(str1, str2)
    size1, size2 = len(str1), len(str2)
    subseq = []

    i, j = 0, 0
    while i < size1 and j < size2:
        if str1[i] == str2[j]:
            subseq.append(str1[i])
            i = i + 1
            j = j + 1
        elif table[i+1][j] >= table[i][j+1]:
            i = i + 1
        else:
            j = j + 1

    return ''.join(subseq)
        

def CreateLengthTable(str1, str2):
    size1, size2 = len(str1), len(str2)
    table = [[0 for x in range(size2 + 1)] for x in range(size1 + 1)]
   
    for i in range(size1 -1 , -1, -1):
        for j in range(size2 - 1, -1, -1):
            if str1[i] == ' ' or str2[j] == ' ':
                table[i][j] = 0
            elif str1[i] == str2[j]:
                table[i][j] = table[i+1][j+1] + 1
            else:
                table[i][j] = max(table[i+1][j], table[i][j+1])
    
    return table

if __name__ == '__main__':
    unittest.main()
