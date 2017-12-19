def pipe_letter_order(grid):
    start = -1
    for i, c in enumerate(grid[0]):
        if c == '|':
            start = i

    direction = (0, 1)
    location = (start, 0)
    letters = []
    count = 0
    while True:
        x, y = location
        if grid[y][x] == '+':
            if direction[0]:
                up = grid[y+1][x] if y+1 < len(grid) else None
                if up is not None and (up == '|' or up.isalpha()):
                    direction = (0, 1)
                else:
                    direction = (0, -1)
            else:
                right = grid[y][x+1] if x+1 < len(grid[y]) else None
                if right is not None and right == '-' or right.isalpha():
                    direction = (1, 0)
                else:
                    direction = (-1, 0)
        elif grid[y][x] == chr(32):
            return letters, count
        elif grid[y][x] not in '-|':
            letters.append(grid[y][x])
        location = (x + direction[0], y + direction[1])
        count += 1


if __name__ == '__main__':
    with open('19.txt') as f:
        grid = f.readlines()
    p1_letters, p2_count = pipe_letter_order(grid)
    print("Part 1: {}".format(''.join(p1_letters)))
    print("Part 2: {}".format(p2_count))
