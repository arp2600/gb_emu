#!/usr/bin/env python3
import sys
from termcolor import cprint


class RingBuffer:
    def __init__(self, size):
        self.buffer = [None] * size
        self.index = 0
        self.iter_index = 0
        self.iter_count = 0

    def reset(self):
        for i in range(0, len(self.buffer)):
            self.buffer[i] = None

    def append(self, item):
        self.buffer[self.index] = item
        self.index = self._inc_index(self.index)

    def _inc_index(self, x):
        return (x + 1) % len(self.buffer)

    def __iter__(self):
        self.iter_index = self.index
        self.iter_count = 0
        return self

    def __next__(self):
        if self.iter_count == len(self.buffer):
            raise StopIteration
        else:
            self.iter_count += 1
            item = self.buffer[self.iter_index]
            self.iter_index = self._inc_index(self.iter_index)
            if item:
                return item
            else:
                return self.__next__()


class PrintDiff:
    def __init__(self, line_buffer):
        self.line_buffer = line_buffer
        for item in self.line_buffer:
            self.print_line(*item)
        self.line_buffer.reset()
        self.update_count = 0

    def update(self, line_no, line1, line2):
        self.update_count += 1
        self.print_line(line_no, line1, line2)

        if are_equal(line1, line2):
            print('')
            return CheckForDiff(self.line_buffer)
        elif self.update_count > 50:
            cprint("Exceeded max diff size", 'red')
            return None
        else:
            return self

    def print_line(self, line_no, line1, line2):
        if are_equal(line1, line2):
            colour = 'green'
            diffchar = ' '
        else:
            colour = 'red'
            diffchar = '*'
        cprint("{:<6}|{}|    {:<50}|    {}".format(line_no, diffchar, line1,
                                                   line2), colour)


class CheckForDiff:
    def __init__(self, line_buffer):
        self.line_buffer = line_buffer

    def update(self, line_no, line1, line2):
        self.line_buffer.append((line_no, line1, line2))

        if not are_equal(line1, line2):
            # for x in self.line_buffer:
            #     print(x)
            return PrintDiff(self.line_buffer)
        else:
            return self


def read_lines(path):
    with open(path, 'r') as f:
        for line in f:
            yield line.rstrip()


def break_line(line):
    opc, rest = line.split(maxsplit=1)
    _, regs = rest.split('#', maxsplit=1)
    return (opc, regs)


def are_equal(line1, line2):
    o1, r1 = break_line(line1)
    o2, r2 = break_line(line2)
    return o1 == o2 and r1 == r2


def main():
    diff_limit = 10
    f1 = sys.argv[1]
    f2 = sys.argv[2]

    state = CheckForDiff(RingBuffer(4))
    state_change_count = 0
    for line_no, (l1, l2) in enumerate(zip(read_lines(f1), read_lines(f2))):
        line_no += 1
        new_state = state.update(line_no, l1, l2)
        if new_state != state:
            state_change_count += 1
            if state_change_count >= (diff_limit * 2):
                cprint("\nDiff limit reached", 'red')
                break
            state = new_state

        if state is None:
            break


if __name__ == '__main__':
    main()
