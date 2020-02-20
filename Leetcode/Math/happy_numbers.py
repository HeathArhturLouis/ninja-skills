class Solution:
    def isHappy(self, n: int) -> bool:
        dic = {}
        while n:
            if 1 in dic:
                return True
            if n in dic:
                return False
            dic[n] = 0
            tmp = 0
            while n:
                tmp += (n%10)**2
                n //= 10
            n = tmp