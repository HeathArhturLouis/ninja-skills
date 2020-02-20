class MultiStacks:
    def __init__(self, no_stacks, stack_size):
        if no_stacks > stack_size:
            print("Too many stacks, try less or add more total memory")
            return None

        self.values = [0] * stack_size
        self.stacks = []

        div = int(stack_size / no_stacks + 0.5)
        for i in range(no_stacks):
            self.stacks.append(SingleStackPointer(i * div))

    def push(self, stack_no, value):
        print("Push ", stack_no, value)
        if stack_no >= len(self.stacks) or stack_no < 0:
            print("Invalid stack no")

        elif stack_no == len(self.stacks) - 1:
            if self.stacks[stack_no].current_index < len(self.values) - 1:
                self.stacks[stack_no].current_index += 1
                self.values[self.stacks[stack_no].current_index] = value

        elif self.stacks[stack_no].current_index < self.stacks[stack_no + 1].starting_index - 1:
            self.stacks[stack_no].current_index += 1
            self.values[self.stacks[stack_no].current_index] = value

        else:
            print("Chosen stack full")

    def pop(self, stack_no):
        print("Pop ", stack_no)
        if stack_no >= len(self.stacks) or stack_no < 0:
            print("Invalid stack no")
            return None

        elif self.stacks[stack_no].current_index > self.stacks[stack_no].starting_index:
            to_return = self.values[self.stacks[stack_no].current_index]
            self.values[self.stacks[stack_no].current_index] = 0
            self.stacks[stack_no].current_index -= 1
            return to_return

        elif self.stacks[stack_no].current_index == self.stacks[stack_no].starting_index:
            if self.values[self.stacks[stack_no].current_index] == 0:
                print("Chosen stack empty")
            else:
                to_return = self.values[self.stacks[stack_no].current_index]
                self.values[self.stacks[stack_no].current_index] = 0
                return to_return

    def print_stacks(self):
        starting = set(stack.starting_index for stack in self.stacks if stack.starting_index)
        for i, val in enumerate(self.values):
            if i in starting:
                print("|", val, end=" ")
            else:
                print(val, end=" ")
        print("")

    def available_stack(self):
        for i in range(len(self.stacks) - 1):
            if self.stacks[i].current_index < self.stacks[i + 1].starting_index:
                return i

        print("All stacks full")
        return None


class SingleStackPointer:
    def __init__(self, starting_index):
        self.starting_index = starting_index
        self.current_index = starting_index - 1


if __name__ == "__main__":
    multi = MultiStacks(3, 8)

    multi.pop(0)
    multi.push(1, 1)
    multi.push(1, 2)
    multi.push(2, 3)
    multi.push(1, 3)
    multi.print_stacks()
    multi.pop(1)
    multi.push(1, 4)
    multi.pop(1)
    multi.push(3, 10)
    multi.print_stacks()
    multi.push(0, 2)
    multi.pop(0)
    multi.push(2, 4)
    multi.pop(2)
    multi.print_stacks()
    print(multi.available_stack())
