from sys import maxsize 
def maxSubArraySum(a): 
       
    max_so_far = -maxsize - 1
    max_ending_here = 0
       
    for i in range(0, len(a)): 
        max_ending_here = max_ending_here + a[i] 
        if (max_so_far < max_ending_here): 
            max_so_far = max_ending_here 
  
        if max_ending_here < 0: 
            max_ending_here = 0   
    return max_so_far 

def maxSubArray(nums):
    largest = 0
    current = 0
    for i in range(len(nums)):
        current = current + nums[i]
        if current > largest:
            largest = current
        if current < 0:
            current = 0
    return largest


print (maxSubArraySum([-2,1,-3,4,-1,2,1,-5,4]))
print (maxSubArray([-2,1,-3,4,-1,2,1,-5,4]))