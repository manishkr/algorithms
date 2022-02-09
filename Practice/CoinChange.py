import unittest

class CoinChangeTest(unittest.TestCase):
    def testCase1(self):
        self.assertEqual(6, CoinChangeRecursive(5, [1,2,3,4]))
        self.assertEqual(6, CoinChangeMemoization(5, [1,2,3,4]))

    def testCase2(self):
        self.assertEqual(1, CoinChangeRecursive(1, [1,2,3]))
        self.assertEqual(1, CoinChangeMemoization(1, [1,2,3]))

    def testCase3(self):
        self.assertEqual(2, CoinChangeRecursive(2, [1,2,3]))
        self.assertEqual(2, CoinChangeMemoization(2, [1,2,3]))

    def testCase4(self):
        self.assertEqual(1, CoinChangeRecursive(0, [1,2,3]))
        self.assertEqual(1, CoinChangeMemoization(0, [1,2,3]))

def CoinChangeRecursive(n, coins):
    return DoCoinChangeRecursive(n, len(coins), coins)

def DoCoinChangeRecursive(n, m, coins):
    if n == 0:
        return 1
    
    if not ((n < 0) or (m <= 0 and n >= 1)):
        return DoCoinChangeRecursive(n, m-1, coins) + DoCoinChangeRecursive(n-coins[m-1], m, coins)

    return 0

def CoinChangeMemoization(n, coins):
    return DoCoinChangeMemoization(n, len(coins), coins)

def DoCoinChangeMemoization(n, m, coins):
    table = [[0 for x in range(m)] for x in range(n + 1)]

    for i in range(m):
        table[0][i] = 1

    for i in range(1, n+1):
        for j in range(m):
            x, y = 0, 0
            if i - coins[j] >= 0:
                x = table[i-coins[j]][j]

            if j >= 1:
                y = table[i][j-1]

            table[i][j] = x + y


    return table[n][m-1]

if __name__ == '__main__':
    unittest.main()
