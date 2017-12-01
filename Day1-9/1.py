def captcha(captcha_input):
    result = 0
    for i in range(len(captcha_input)):
        next_char = captcha_input[(i+1)%len(captcha_input)]
        if captcha_input[i] == next_char:
            result += int(next_char)
    return result


def captcha2(captcha_input):
    result = 0
    n = len(captcha_input)
    for i in range(len(captcha_input)):
        next_char = captcha_input[(i + n//2)%len(captcha_input)]
        if captcha_input[i] == next_char:
            result += int(next_char)
    return result


if __name__ == '__main__':
    with open('1.txt') as f:
        puzzle = f.read().strip()
    assert captcha('1122') == 3
    assert captcha('1111') == 4
    assert captcha('1234') == 0
    assert captcha('91212129') == 9
    part1 = captcha(puzzle)
    print("Part 1: {}".format(part1))

    part2 = captcha2(puzzle)
    print("Part 2: {}".format(part2))
