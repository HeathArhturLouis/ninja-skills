import random
class Solution:

    def __init__(self, nums):
        self.nums = nums.copy()
        self.shuffled = nums
        

    def reset(self):
        """
        Resets the array to its original configuration and return it.
        """
        return self.nums
        
        

    def shuffle(self):
        """
        Returns a random shuffling of the array.
        """
        random.shuffle(self.shuffled)
        return self.shuffled

if __name__ == "__main__":
    sol = Solution([1,2,3,5])
    print(sol.shuffle())
    print(sol.reset())
    print(sol.shuffle())

    print ([0] * 5)