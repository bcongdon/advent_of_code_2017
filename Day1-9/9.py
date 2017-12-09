def stream_score(stream):
    score, chars = 0, 0
    idx = 0
    stack = [0]
    while idx < len(stream):
        if stream[idx] == '{':
            score += stack[-1] + 1
            stack.append(len(stack))
            idx += 1
        elif stream[idx] == '}':
            stack.pop()
            idx += 1
        elif stream[idx] == '<':
            g_len, g_chars = garbage_length(stream[idx:])
            idx += g_len
            chars += g_chars
        else:
            idx += 1
    return score, chars


def garbage_length(stream):
    '''
    Given a stream of format "<..."
    calculate the length of the garbage group
    '''
    chars = 0
    idx = 1
    while idx < len(stream):
        if stream[idx] == '>':
            break
        elif stream[idx] == '!':
            idx += 2
        else:
            idx += 1
            chars += 1
    return idx + 1, chars


if __name__ == '__main__':
    # assert garbage_length('<random characters>') == len('<random characters>')
    # assert garbage_length('<<<<>') == len('<<<<>')
    # assert garbage_length('<{!>}>') == len('<{!>}>')
    # assert garbage_length('<!!>') == len('<!!>')
    # assert garbage_length('<!!!>>') == len('<!!!>>')
    # assert garbage_length('<{o"i!a,<{i<a>') == len('<{o"i!a,<{i<a>')

    # assert stream_score('{}') == 1
    # assert stream_score('{{{}}}') == 6
    # assert stream_score('{{},{}}') == 5
    # assert stream_score('{{<a!>},{<a!>},{<a!>},{<ab>}}') == 3

    with open('9.txt') as f:
        stream = f.read().strip()
    score, chars = stream_score(stream)
    print("Part 1: {}".format(score))
    print("Part 2: {}".format(chars))
