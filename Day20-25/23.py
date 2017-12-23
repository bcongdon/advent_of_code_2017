from collections import defaultdict


class VM:
    def __init__(self, instructions, registers=None):
        self.instructions = instructions
        self.pc = 0

        if registers is None:
            registers = defaultdict(int)
        self.registers = registers
        self.num_mul_ops = 0

    def _get_value(self, operand):
        try:
            return int(operand)
        except:
            return self.registers[operand]

    def run_instruction(self):
        inst = instructions[self.pc]
        if inst[0] == 'set':
            self.registers[inst[1]] = self._get_value(inst[2])
        elif inst[0] == 'sub':
            self.registers[inst[1]] -= self._get_value(inst[2])
        elif inst[0] == 'mul':
            self.num_mul_ops += 1
            self.registers[inst[1]] *= self._get_value(inst[2])
        elif inst[0] == 'jnz':
            if self._get_value(inst[1]) != 0:
                self.pc += self._get_value(inst[2])
                return
        self.pc += 1


def part2(instructions):
    start, end, skip = 0, 0, 0
    for inst in instructions:
        if inst[0] == 'set' and inst[1] == 'b':
            start = int(inst[2]) * 100 + 100000
        elif inst[0] == 'sub' and inst[1] == 'c':
            end = start - int(inst[2])
        elif inst[0] == 'sub' and inst[1] == 'b' and end:
            skip = -int(inst[2])

    h = 0
    for x in range(start, end+1, skip):
        for j in range(2, int(x**0.5)):
            if x % j == 0:
                h += 1
                break
    return h

if __name__ == '__main__':
    with open('23.txt') as f:
        instructions = [l.split() for l in f.readlines()]
    P1 = VM(instructions)
    while P1.pc < len(instructions):
        P1.run_instruction()
    print("Part 1: {}".format(P1.num_mul_ops))

    print("Part 2: {}".format(part2(instructions)))
