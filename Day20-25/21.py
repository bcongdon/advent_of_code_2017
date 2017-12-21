import numpy as np


def to_cfg_string(chunk):
    return '/'.join([''.join(['#' if b else '.' for b in r]) for r in chunk])


def from_cfg_string(cfg_str):
    rows = cfg_str.split('/')
    return np.array([[True if j == '#' else False for j in i] for i in rows])


class Matcher:
    def __init__(self, rules):
        self.rules = rules
        self.memoized = {}

    def find_new_config(self, chunk):
        cfg_str = to_cfg_string(chunk)
        if cfg_str in self.memoized:
            return self.memoized[cfg_str]

        for _ in range(4):
            if to_cfg_string(chunk) in rules:
                new_cfg = from_cfg_string(rules[to_cfg_string(chunk)])
                self.memoized[cfg_str] = new_cfg
                return new_cfg

            flipped = np.flip(chunk, 0)
            if to_cfg_string(flipped) in rules:
                new_cfg = from_cfg_string(rules[to_cfg_string(flipped)])
                self.memoized[cfg_str] = new_cfg
                return new_cfg
            chunk = np.rot90(chunk)


def iterate_production(config, matcher):
    chunk_size = 0
    if len(config) % 2 == 0:
        chunk_size = 2
    else:
        chunk_size = 3
    new_size = (chunk_size+1)*len(config)//chunk_size

    new_cfg = np.empty((new_size, new_size), dtype=bool)
    for x in range(0, len(config), chunk_size):
        cx = (x//chunk_size)*(chunk_size+1)
        for y in range(0, len(config), chunk_size):
            cy = (y//chunk_size)*(chunk_size+1)
            cfg = matcher.find_new_config(config[x:x+chunk_size, y:y+chunk_size])
            new_cfg[cx:cx+chunk_size+1, cy:cy+chunk_size+1] = cfg
    return new_cfg


def count_lights(config):
    return(list(config.flatten()).count(True))

if __name__ == '__main__':
    with open('21.txt') as f:
        lines = f.readlines()
    rules = {k: v for k, v in [l.strip().split(" => ") for l in lines]}

    matcher = Matcher(rules)
    config = from_cfg_string('.#./..#/###')
    for i in range(5):
        config = iterate_production(config, matcher)
    print("Part 1: {}".format(count_lights(config)))

    for i in range(18-5):
        config = iterate_production(config, matcher)
    print("Part 2: {}".format(count_lights(config)))
