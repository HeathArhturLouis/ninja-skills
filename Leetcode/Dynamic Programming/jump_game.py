def canJump(nums):
    if len(nums) == 1:
        return True
    last_good_index = len(nums) - 1
    for i in range(len(nums)-1, -1, -1):
        if i + nums[i] >= last_good_index:
            last_good_index = i
    return last_good_index == 0

if __name__ == "__main__":
    print(canJump([2,0,0]))