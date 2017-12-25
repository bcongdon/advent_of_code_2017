from collections import defaultdict

turing_machine = {
    'a': [(1, +1, 'b'), (0, -1, 'f')],
    'b': [(0, +1, 'c'), (0, +1, 'd')],
    'c': [(1, -1, 'd'), (1, +1, 'e')],
    'd': [(0, -1, 'e'), (0, -1, 'd')],
    'e': [(0, +1, 'a'), (1, +1, 'c')],
    'f': [(1, -1, 'a'), (1, +1, 'a')],
}


def run_tm(steps):
    tape = defaultdict(int)
    loc = 0
    state = 'a'

    for _ in range(steps):
        val = tape[loc]
        new_val, mvt, new_state = turing_machine[state][val]
        tape[loc] = new_val
        loc += mvt
        state = new_state
    return sum(tape.values())

if __name__ == '__main__':
    steps = 12994925
    print("Part 1: {}".format(run_tm(steps)))
    print("Part 2: {}".format("⭐"))
