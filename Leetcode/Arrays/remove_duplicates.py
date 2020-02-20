class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        leng = 0
        nums.sort()
        for i in range(len(nums)):
            if nums[leng] != nums[i]:
                leng += 1
                nums[leng] = nums[i]
        return leng+1