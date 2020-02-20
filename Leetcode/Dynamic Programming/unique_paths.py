def foo():
    return -1

def paths(m,n):
    cache = [[foo() for i in range(m)] for j in range(n)]

    for row in cache:
        row[-1] = 1
    
    for i in range(len(cache[-1])):
        cache[-1][i] = 1

    cache[-1][-1] = 0
    
    for col in range(m-2,-1,-1):
        for row in range(n-2,-1,-1):
            cache[row][col] = cache[row+1][col] + cache[row][col+1]
    return cache[0][0]

if __name__ == "__main__":
    paths(7,3)