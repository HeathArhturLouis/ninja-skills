def check_palindrome(self):
    node = self.head

    if not node:
        return True

    stack = []
    while node:
        stack.append(node.val)
        node = node.next

    node = self.head
    for i in range(len(stack)-1, -1, -1):  # You can stop at halfway
        if not stack[i] == node.val:
            return False
        node = node.next
    return True
