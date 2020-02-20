def climbStairs(n: int) -> int:
    a = 1
    b = 1
    if n <= 1:
        return 1
    else:
        for _ in range(2, n+1):
            c = a + b
            a, b = b, c
        return c


print(climbStairs(3))
