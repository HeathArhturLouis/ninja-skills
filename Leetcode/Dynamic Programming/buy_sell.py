def maxProfit_brute(prices):
    if not prices:
        return 0
    best = 0
    for left in range(len(prices) - 1):
        for right in range(left + 1, len(prices)):
            if prices[right] - prices[left] > best:
                best = prices[right] - prices[left]
    return best



print(maxProfit([2,7,1,4]))
