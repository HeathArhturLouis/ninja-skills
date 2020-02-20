def rotate_symmetries(arr):
    for row in arr:
        row.reverse()
    N = len(arr) - 1
    for i in range(N):
        print(N+1 - i)
        for j in range(N+1 - i):
            arr[i][j], arr[N-j][N-i] = arr[N-j][N-i], arr[i][j]
    return arr


def rotate_pythonic(arr):
    return zip(*arr[::-1])


def rotate_np(arr):
    import numpy as np
    return np.rot90(arr, k=1, axes=(1, 0))


if __name__ == "__main__":
    N = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
    ]

    for i in rotate_symmetries(N):
        print(i)
    N = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
    ]
    print("")
    print(rotate_np(N))
