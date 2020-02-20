'''
Queue via Stacks: Implement a MyQueue class which implements a queue using two stacks.
'''

class myQueue:
    '''
    Fast pops and peeks, slow enqueues tho
    '''
    def __init__(self):
        self.stackA = []
        self.stackB = []

    def enqueue(self, item):
        '''
        O(N)
        '''
        while self.stackA:
            self.stackB.append(self.stackA.pop())
        self.stackA.append(item)
        while self.stackB:
            self.stackA.append(self.stackB.pop())

    def dequeue(self):
        '''
        O(1)
        '''
        return self.stackA.pop()
    def peek(self):
        '''
        O(1)
        '''
        # peek stackA
        return self.stackA[-1]
    def isEmpty(self):
        '''
        O(1)
        '''
        return (self.stackA == [])

class myQueue2:
    '''
    Fast enqueue, slow peeks and pops
    A bit worse, since peeking is harder
    '''
    def __init__(self):
        self.stackA = []
        self.stackB = []

    def enqueue(self, item):
        '''
        O(1)
        '''
        self.stackA.append(item)

    def dequeue(self):
        '''
        O(N)
        '''
        while self.stackA:
            self.stackB.append(self.stackA.pop())
        item = self.stackB.pop()
        while self.stackB:
            self.stackA.append(self.stackB.pop())
        return item

    def peek(self):
        '''
        O(N)
        '''
        # peek stackA
        while self.stackA:
            self.stackB.append(self.stackA.pop())
        item = self.stackB[-1]
        while self.stackB:
            self.stackA.append(self.stackB.pop())

        return item

    def isEmpty(self):
        '''
        O(1)
        '''
        return (self.stackA == [])



Q = myQueue2()
Q.enqueue('A')
Q.enqueue('B')
Q.enqueue('C')
Q.enqueue('D')
Q.enqueue('E')

print(Q.isEmpty())
print(Q.dequeue())
print(Q.dequeue())
print(Q.dequeue())
print(Q.peek())

print(Q.dequeue())
print(Q.dequeue())


