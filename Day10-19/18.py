from collections import defaultdict


class VM:
    def __init__(self, instructions, registers=None, part1=False):
        self.instructions = instructions
        self.pc = 0
        self.send_val = None
        self.receive_queue = []

        if registers is None:
            registers = defaultdict(int)
        self.registers = registers

        self.part1 = part1
        self.first_receive = None

    def _get_value(self, operand):
        try:
            return int(operand)
        except:
            return self.registers[operand]

    def run_instruction(self):
        inst = instructions[self.pc]
        if inst[0] == 'snd':
            self.last_sound = self._get_value(inst[1])
            self.send_val = self._get_value(inst[1])
        elif inst[0] == 'set':
            self.registers[inst[1]] = self._get_value(inst[2])
        elif inst[0] == 'add':
            self.registers[inst[1]] += self._get_value(inst[2])
        elif inst[0] == 'mul':
            self.registers[inst[1]] *= self._get_value(inst[2])
        elif inst[0] == 'mod':
            self.registers[inst[1]] %= self._get_value(inst[2])
        elif inst[0] == 'rcv':
            if self.part1:
                self.first_receive = self.last_sound
            else:
                if self.receive_queue:
                    self.registers[inst[1]] = self.receive_queue.pop()
                else:
                    return
        elif inst[0] == 'jgz':
            if self._get_value(inst[1]) > 0:
                self.pc += self._get_value(inst[2])
                return
        self.pc += 1


def make_process_reg(pid):
    reg = defaultdict(int)
    reg['p'] = pid
    return reg


def run_vm_pair(instructions):
    P0 = VM(instructions, make_process_reg(0))
    P1 = VM(instructions, make_process_reg(1))
    part2 = 0
    pc0, pc1 = -1, -1
    while P0.pc != pc0 or P1.pc != pc1:
        pc0 = P0.pc
        pc1 = P1.pc
        if P0.send_val is not None:
            P1.receive_queue = [P0.send_val] + P1.receive_queue
            P0.send_val = None
        if P1.send_val is not None:
            P0.receive_queue = [P1.send_val] + P0.receive_queue
            P1.send_val = None
            part2 += 1
        P0.run_instruction()
        P1.run_instruction()
    return part2

if __name__ == '__main__':
    with open('18.txt') as f:
        instructions = [l.split() for l in f.readlines()]
    P1 = VM(instructions, part1=True)
    while P1.first_receive is None or P1.first_receive == 0:
        P1.run_instruction()
    print("Part 1: {}".format(P1.first_receive))

    part2 = run_vm_pair(instructions)
    print("Part 2: {}".format(part2))
