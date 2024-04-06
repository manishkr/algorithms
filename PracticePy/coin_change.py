def coin_change(coins, amount):
    dp = [float('inf')] * (amount + 1)
    dp[0] = 0

    for coin in coins:
        for i in range(coin, amount + 1):
            dp[i] = min(dp[i], dp[i - coin] + 1)
            print(i,dp[i])

    return dp[amount] if dp[amount] != float('inf') else -1

if __name__ == '__main__':
    x = [1,2,5]
    print(coin_change(x, 11))
