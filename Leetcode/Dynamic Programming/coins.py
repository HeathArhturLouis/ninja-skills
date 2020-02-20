import sys
def make_change(coins, amount):
    if amount <= 0 or not coins:
        return 0
    cache = [0] * (amount + 1)

    for i in range(1, len(cache)):
        best = sys.maxsize
        for c in coins:
            if i >= c:
                curr = cache[i - c] + 1
                if curr < best:
                    best = curr
        cache[i] = best
    return cache[-1]
    


if __name__ == "__main__":
    print(make_change([1, 3, 5], 17))
    