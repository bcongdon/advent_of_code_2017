import operator
from collections import defaultdict

instructions = {
    'inc': operator.add,
    'dec': operator.sub
}

comparators = {
    '>': operator.gt,
    '<': operator.lt,
    '>=': operator.ge,
    '<=': operator.le,
    '==': operator.eq,
    '!=': operator.ne
}


def run_instructions(instructions):
    registers = defaultdict(int)
    max_at_runtime = 0
    for i in instructions:
        asn_reg, op, amt, cmp_reg, cmp_op, cmp_amt = i
        if cmp_op(registers[cmp_reg], cmp_amt):
            registers[asn_reg] = op(registers[asn_reg], amt)
        max_at_runtime = max(max_at_runtime, registers[asn_reg])
    max_at_end = max(registers.values())
    return (max_at_end, max_at_runtime)


def parse_instruction(line):
    tokens = line.split()
    return (
        tokens[0],
        instructions[tokens[1]],
        int(tokens[2]),
        tokens[4],
        comparators[tokens[5]],
        int(tokens[6])
    )


if __name__ == '__main__':
    with open('8.txt') as f:
        lines = f.readlines()
    instructions = [parse_instruction(l) for l in lines]
    max_at_end, max_at_runtime = run_instructions(instructions)
    print("Part 1: {}".format(max_at_end))
    print("Part 2: {}".format(max_at_runtime))