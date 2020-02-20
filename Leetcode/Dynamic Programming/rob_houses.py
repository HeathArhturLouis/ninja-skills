def robb(nums):
    if len(nums) == 1:
        return nums[0]
    elif len(nums) == 2:
        return max(nums[0], nums[1])

    nums[2] += nums[0]

    for i in range(3, len(nums)):
        nums[i] += max(nums[i-2], nums[i-3])

    return max(nums[-1], nums[-2])


if __name__ == "__main__":
    arr = [2, 4, 3]
    print(robb(arr))

